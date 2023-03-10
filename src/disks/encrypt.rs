use crate::prelude::*;

pub fn root_partition(c: Config) -> Config {
    let root_part: String = get_value(&c, "disk.root_partition");

    if !get_value::<bool>(&c, "disk.encryption.enable") {
        return add_value(c, "disk.root_container", root_part);
    }

    let method: String = get_value(&c, "disk.encryption.method");
    if method == "password" {
        let rc = encrypt_pw(&root_part);
        return add_value(c, "disk.root_container", rc);
    } else if method == "keydrive" {
        let kd_target: String = get_value(&c, "disk.encryption.key");
        let hostname: String = get_value(&c, "hostname");
        let (keypart, keydrive) = create_keydrive(&kd_target, &hostname);

        let (rc, file) = encrypt_kd(&keydrive, &hostname, &root_part);
        let c = add_value(c, "disk.root_container", rc);
        let c = add_value(c, "disk.keydrive.partition", keypart);
        let c = add_value(c, "disk.keydrive.file", file);

        return c;
    } else {
        Error::Config("Improper encryption method supplied".into()).handle()
    }
}

fn encrypt_pw(target: &str) -> String {
    let cont = "cryptvol";
    password_encrypt(target, cont);
    "/dev/mapper/".to_string() + cont
}

fn encrypt_kd(keyd: &str, hostname: &str, target: &str) -> (String, String) {
    shrun(&ShellCommand::new("mkdir").args(["-p", "/tmp/keydrive"]));
    shrun(&ShellCommand::new("mount").args([&keyd, "/tmp/keydrive"]));
    let keyfile = String::from("/") + hostname + ".key";

    shrun(&ShellCommand::new("cryptsetup").args([
        "luksFormat",
        "-c",
        "aes-xts-plain64",
        "-h",
        "sha512",
        "-s",
        "512",
        &target,
        &format!("/tmp/keydrive{}", &keyfile),
    ]));

    shrun(&ShellCommand::new("cryptsetup").args([
        "open",
        "--type",
        "luks",
        "-d",
        &format!("/tmp/keydrive{}", &keyfile),
        &target,
        "cryptvol",
    ]));

    shrun(&ShellCommand::new("umount").args(["/tmp/keydrive"]));
    ("/dev/mapper/cryptvol".into(), keyfile)
}

fn create_keydrive(target: &str, name: &str) -> (String, String) {
    let map = "keydrive";

    let formatter_string = String::from("g\nn\n\n\n+256M\nw\n");
    shrun(
        &ShellCommand::new("fdisk")
            .pipe_string(formatter_string)
            .args([target]),
    );
    let kd_result = shrun(&ShellCommand::new("fdisk").args(["-l", target]));
    let grep_kd = shrun(
        &ShellCommand::new("grep")
            .pipe_string(&kd_result)
            .args(["Linux filesystem"]),
    );

    let kd_part = Vec::from_iter(grep_kd.split_whitespace())[0].to_string();

    password_encrypt(&kd_part, map);
    let kd_target = String::from("/dev/mapper/") + map;

    shrun(&ShellCommand::new("mkdir").args(["-p", "/tmp/keydrive"]));
    shrun(&ShellCommand::new("mkfs.vfat").args(["-F32", &kd_target]));
    shrun(&ShellCommand::new("mount").args([&kd_target, "/tmp/keydrive"]));

    create_keyfile("/tmp/keydrive", &("/".to_string() + name + ".key"));

    shrun(&ShellCommand::new("umount").args(["/tmp/keydrive"]));
    (kd_part, kd_target)
}

fn create_keyfile(location: &str, name: &str) {
    shrun(&ShellCommand::new("dd").args(&[
        "if=/dev/urandom",
        "bs=1",
        "count=512",
        &format!("of={}{}", location, name),
    ]));
}

fn password_encrypt(target: &str, mount: &str) {
    let prompt = format!("Input new password for {} >>: ", target);
    let pw = rpassword::prompt_password(prompt);
    let pw = pw.expect("Failed to prompt for password");

    shrun(&ShellCommand::new("cryptsetup").pipe_string(&pw).args([
        "luksFormat",
        "-c",
        "aes-xts-plain64",
        "-h",
        "sha512",
        "-s",
        "512",
        target,
    ]));

    shrun(
        &ShellCommand::new("cryptsetup")
            .pipe_string(&pw)
            .args(["open", "--type", "luks", target, mount]),
    );
}
