use once_cell::sync::Lazy;
use regex::Regex;

pub type Issue = Option<(String, String)>;

pub fn find_issues(log: &str) -> Vec<(String, String)> {
    let issues = [
        fabric_internal,
        flatpak_nvidia,
        forge_java,
        intel_hd,
        java_option,
        macos_ns,
        oom,
        optinofine,
        outdated_launcher,
        wrong_java,
    ];

    issues.iter().filter_map(|issue| issue(log)).collect()
}

fn fabric_internal(log: &str) -> Issue {
    const CLASS_NOT_FOUND: &str = "Caused by: java.lang.ClassNotFoundException: ";

    let issue = (
        "Fabric Internal Access".to_string(),
        "The mod you are using is using fabric internals that are not meant \
        to be used by anything but the loader itself.
        Those mods break both on Quilt and with fabric updates.
        If you're using fabric, downgrade your fabric loader could work, \
        on Quilt you can try updating to the latest beta version, \
        but there's nothing much to do unless the mod author stops using them."
            .to_string(),
    );

    let errors = [
        &format!("{CLASS_NOT_FOUND}net.fabricmc.fabric.impl"),
        &format!("{CLASS_NOT_FOUND}net.fabricmc.fabric.mixin"),
        &format!("{CLASS_NOT_FOUND}net.fabricmc.fabric.loader.impl"),
        &format!("{CLASS_NOT_FOUND}net.fabricmc.fabric.loader.mixin"),
        "org.quiltmc.loader.impl.FormattedException: java.lang.NoSuchMethodError:",
    ];

    let found = errors.iter().any(|e| log.contains(e));
    found.then_some(issue)
}

fn flatpak_nvidia(log: &str) -> Issue {
    let issue = (
        "Outdated Nvidia Flatpak Driver".to_string(),
        "The Nvidia driver for flatpak is outdated.
        Please run `flatpak update` to fix this issue. \
        If that does not solve it, \
        please wait until the driver is added to Flathub and run it again."
            .to_string(),
    );

    let found = log.contains("org.lwjgl.LWJGLException: Could not choose GLX13 config")
        || log.contains("GLFW error 65545: GLX: Failed to find a suitable GLXFBConfig");

    found.then_some(issue)
}

fn forge_java(log: &str) -> Issue {
    let issue = (
        "Forge Java Bug".to_string(),
        "Old versions of Forge crash with Java 8u321+.
            To fix this, update forge to the latest version via the Versions tab
            (right click on Forge, click Change Version, and choose the latest one)
            Alternatively, you can download 8u312 or lower. \
            See [archive](https://github.com/adoptium/temurin8-binaries/releases/tag/jdk8u312-b07)"
            .to_string(),
    );

    let found = log.contains("java.lang.NoSuchMethodError: sun.security.util.ManifestEntryVerifier.<init>(Ljava/util/jar/Manifest;)V");
    found.then_some(issue)
}

fn intel_hd(log: &str) -> Issue {
    let issue =
        (
        "Intel HD Windows 10".to_string(),
        "Your drivers don't support windows 10 officially
        See https://prismlauncher.org/wiki/getting-started/installing-java/#a-note-about-intel-hd-20003000-on-windows-10 for more info".to_string()
    );

    let found = log.contains("java.lang.NoSuchMethodError: sun.security.util.ManifestEntryVerifier.<init>(Ljava/util/jar/Manifest;)V");
    found.then_some(issue)
}

fn java_option(log: &str) -> Issue {
    static VM_OPTION_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"Unrecognized VM option '(.+)'[\r\n]").unwrap());
    static OPTION_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"Unrecognized option: (.+)[\r\n]").unwrap());

    if let Some(captures) = VM_OPTION_REGEX.captures(log) {
        let title = if &captures[1] == "UseShenandoahGC" {
            "Wrong Java Arguments"
        } else {
            "Java 8 and below don't support ShenandoahGC"
        };
        return Some((
            title.to_string(),
            format!("Remove `-XX:{}` from your Java arguments", &captures[1]),
        ));
    }

    if let Some(captures) = OPTION_REGEX.captures(log) {
        return Some((
            "Wrong Java Arguments".to_string(),
            format!("Remove `{}` from your Java arguments", &captures[1]),
        ));
    }

    None
}

fn macos_ns(log: &str) -> Issue {
    let issue = (
    "MacOS NSInternalInconsistencyException".to_string(),
    "You need to downgrade your Java 8 version. See https://prismlauncher.org/wiki/getting-started/installing-java/#older-minecraft-on-macos".to_string()
);

    let found =
        log.contains("Terminating app due to uncaught exception 'NSInternalInconsistencyException");
    found.then_some(issue)
}

fn oom(log: &str) -> Issue {
    let issue = (
        "Out of Memory".to_string(),
        "Allocating more RAM to your instance could help prevent this crash.".to_string(),
    );

    let found = log.contains("java.lang.OutOfMemoryError");
    found.then_some(issue)
}

fn optinofine(log: &str) -> Issue {
    let issue = (
        "Potential OptiFine Incompatibilities".to_string(),
        "OptiFine is known to cause problems when paired with other mods. \
        Try to disable OptiFine and see if the issue persists.
        Check `/tag optifine` for more info & some typically more compatible alternatives you can use."
            .to_string(),
    );

    let found = log.contains("[✔] OptiFine_") || log.contains("[✔] optifabric-");
    found.then_some(issue)
}

// TODO: @TheKodeToad
fn outdated_launcher(_log: &str) -> Issue {
    todo!()
}

fn wrong_java(log: &str) -> Issue {
    static SWITCH_VERSION_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"(?m)Please switch to one of the following Java versions for this instance:[\r\n]+(Java version [\d.]+)",
        ).unwrap()
    });

    if let Some(captures) = SWITCH_VERSION_REGEX.captures(log) {
        let versions = captures[1].split('\n').collect::<Vec<&str>>().join(", ");
        return Some((
            "Wrong Java Version".to_string(),
            format!("Please switch to one of the following: `{versions}`\nFor more information, type `/tag java`"),
        ));
    }

    let issue = (
        "Java compatibility check skipped".to_string(),
        "The Java major version may not work with your Minecraft instance. Please switch to a compatible version".to_string()
    );

    log.contains("Java major version is incompatible. Things might break.")
        .then_some(issue)
}
