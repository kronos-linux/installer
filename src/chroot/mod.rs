use crate::prelude::*;

use std::{
    cmp::min,
    fs::{File, OpenOptions},
    io::Write,
    os::unix::fs,
};

mod download;
mod mounts;

pub fn configure(c: Config) -> Config {
    prepare(&c);
    execute(&c);
    c
}

fn prepare(c: &Config) {
    match std::env::set_current_dir("/mnt/gentoo") {
        Err(_) => Error::Generic("Failed to change directory".into()).handle(),
        _ => (),
    }
    let gui_enable = get_value(c, "gui.enabled");

    let target = download::get_s3_string(gui_enable);
    let target_asc = target.clone() + ".asc";
    debug!("Stage3 target: {}", target);
    debug!("Target asc: {}", target_asc);

    download::stage3(&target, &target_asc);
    make_conf();
    ebuild_repo();
    dns_info();

    mounts::relevant();
}

fn execute(c: &Config) {
    fs::chroot("/mnt/gentoo").expect("Chroot failed");
    std::env::set_current_dir("/").expect("Directory change failed");

    debug!(
        "Inside chroot:\n{}",
        shrun(&ShellCommand::new("ls").args(["-lah"]))
    );

    info!("Mounting boot partition");
    let target: String = get_value(c, "disk.esp");

    shrun(&ShellCommand::new("mount").args([&target, "/boot"]));

    if get_value(c, "disk.swap.enable") {
        let t: String = get_value(c, "disk.swap_volume");
        swap(&t);
        zswap();
    }
}

fn zswap() {
    let mut zs_enable = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/sys/module/zswap/parameters/enabled")
        .expect("Could not open zswap enable");

    let mut zs_compress = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/sys/module/zswap/parameters/compressor")
        .expect("Could not open zswap enable");

    let mut zs_alloc = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/sys/module/zswap/parameters/zpool")
        .expect("Could not open zswap enable");

    write!(zs_compress, "lz4").expect("Could not set lz4 as compressor");
    write!(zs_alloc, "z3fold").expect("Could not set z3fold as allocator");
    write!(zs_enable, "1").expect("Could not enable zswap");

    debug!(
        "ZSwap enabled:\n{}",
        shrun(&ShellCommand::new("grep").args(["-R", ".", "/sys/module/zswap/parameters"]))
    );
}

fn swap(target: &str) {
    info!("Enabling swap");
    shrun(&ShellCommand::new("mkswap").args([target]));
    shrun(&ShellCommand::new("swapon").args([target]));
    debug!(
        "Swap enabled:\n{}",
        shrun(&ShellCommand::new("swapon").args(["--show"]))
    );
}

fn dns_info() {
    shrun(&ShellCommand::new("cp").args(["--dereference", "/etc/resolv.conf", "/mnt/gentoo/etc/"]));
}

fn ebuild_repo() {
    shrun(&ShellCommand::new("mkdir").args(["-pv", "/mnt/gentoo/etc/portage/repos.conf"]));
    shrun(&ShellCommand::new("cp").args([
        "/mnt/gentoo/usr/share/portage/config/repos.conf",
        "/mnt/gentoo/etc/portage/repos.conf/gentoo.conf",
    ]));
}

fn make_conf() {
    info!("Configuring make.conf of the new system");

    let nproc: u16 = shrun(&ShellCommand::new("nproc"))
        .replace("\n", "")
        .parse()
        .expect("Failed to detect CPU count");
    let lsmem = shrun(&ShellCommand::new("lsmem"));
    let mem = shrun(
        &ShellCommand::new("grep")
            .args(["Total online memory"])
            .pipe_string(lsmem),
    );
    let mem: u16 = shrun(
        &ShellCommand::new("grep")
            .pipe_string(mem)
            .args(["-om1", "[0-9]*"]),
    )
    .replace("\n", "")
    .parse()
    .expect("Failed to detect memory amount");

    if mem <= 1 {
        Error::Generic("Insufficient memory to continue installation".into()).handle::<()>();
    }

    debug!("Found CPUs: {}", nproc);
    debug!("Found memory: {}", mem);

    let make_text = String::from("");
    let make_text = add_header(make_text);
    let make_text = add_common_flags(make_text);
    let make_text = add_makeopts(make_text, nproc, mem);
    let make_text = add_portage(make_text);
    let make_text = add_portage_nice(make_text);
    let make_text = add_license(make_text);
    let make_text = add_keywords(make_text);

    shrun(&ShellCommand::new("rm").args(["/mnt/gentoo/etc/portage/make.conf"]));

    let mut file =
        File::create("/mnt/gentoo/etc/portage/make.conf").expect("Could not open make.conf");
    write!(file, "{}", make_text).expect("Could not write make.conf");

    let content = shrun(&ShellCommand::new("cat").args(["/mnt/gentoo/etc/portage/make.conf"]));
    debug!("Finished writing to make.conf:\n{}", content);
}

fn add_header(s: String) -> String {
    let headerl1 =
        "#  ##     ##    ###    ##    ## ########      ######   #######  ##    ## ########\n";
    let headerl2 = "#  ###   ###   ## ##   ##   ##  ##           ##    ## ##     ## ###   ## ##\n";
    let headerl3 = "#  #### ####  ##   ##  ##  ##   ##           ##       ##     ## ####  ## ##\n";
    let headerl4 =
        "#  ## ### ## ##     ## #####    ######       ##       ##     ## ## ## ## ######\n";
    let headerl5 = "#  ##     ## ######### ##  ##   ##           ##       ##     ## ##  #### ##\n";
    let headerl6 = "#  ##     ## ##     ## ##   ##  ##       ### ##    ## ##     ## ##   ### ##\n";
    let headerl7 = "#  ##     ## ##     ## ##    ## ######## ###  ######   #######  ##    ## ##\n";
    let headerl8 = "#\n";
    s + headerl1 + headerl2 + headerl3 + headerl4 + headerl5 + headerl6 + headerl7 + headerl8 + "\n"
}

fn add_common_flags(s: String) -> String {
    let header = "# C, C++ and FORTRAN options for GCC.\n";
    let common = "COMMON_FLAGS=\"-march=native -O2 -pipe\"\n";
    let common_block = "CFLAGS=\"${COMMON_FLAGS}\"\nCXXFLAGS=\"${COMMON_FLAGS}\"\nFCFLAGS=\"${COMMON_FLAGS}\"\nFFLAGS=\"${COMMON_FLAGS}\"\n";

    s + header + common + common_block + "\n"
}

fn add_portage(s: String) -> String {
    let pord = "PORTDIR=\"/var/db/repos/gentoo\"\n";
    let dstd = "DISTDIR=\"/var/cache/distfiles\"\n";
    let pkgd = "PKGDIR=\"/var/cache/binpkgs\"\n";
    let euse = "USE=\"\"\n";
    s + pord + dstd + pkgd + "\n" + euse + "\n"
}

fn add_makeopts(s: String, nproc: u16, mem: u16) -> String {
    let cpu_limit = match nproc {
        1 => 1,
        n => n + 1,
    };
    let mem_limit = (mem / 2) as u16;
    let jobs = min(cpu_limit, mem_limit);
    let load = nproc as f64 * 0.85;

    let mko = format!("MAKEOPTS=\"-j{} -l{}\"\n", jobs, load);
    let edo = format!(
        "EMERGE_DEFAULT_OPTS=\"--jobs={} --load-average={}\"\n",
        jobs, load
    );
    s + &mko + &edo + "\n"
}

fn add_portage_nice(s: String) -> String {
    s + "PORTAGE_NICENESS=\"16\"\n"
}

fn add_license(s: String) -> String {
    s + "ACCEPT_LICENSE=\"* -@EULA\"\n"
}

fn add_keywords(s: String) -> String {
    s + "ACCEPT_KEYWORDS=\"amd64\"\n"
}
