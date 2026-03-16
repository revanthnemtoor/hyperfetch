/// Registry of ASCII logos for various operating systems.
/// Supports auto-detection and custom logo loading from local files.
pub struct AsciiLogo {
    pub lines: Vec<String>,
    pub color: String,
}

impl AsciiLogo {
    pub fn get(os_name: &str) -> Self {
        if os_name.eq_ignore_ascii_case("none") {
            return Self {
                lines: vec![],
                color: "white".to_string(),
            };
        }

        if os_name.starts_with('/') || os_name.starts_with("~/") || os_name.starts_with("./") {
            let path = if os_name.starts_with("~/") {
                if let Some(mut home) = dirs::home_dir() {
                    home.push(os_name.trim_start_matches("~/"));
                    home
                } else {
                    std::path::PathBuf::from(os_name)
                }
            } else {
                std::path::PathBuf::from(os_name)
            };

            if let Ok(content) = std::fs::read_to_string(path) {
                return Self {
                    lines: content.lines().map(|s| s.to_string()).collect(),
                    color: "white".to_string(),
                };
            }
        }

        let name_lower = os_name.to_lowercase();
        if name_lower.contains("arch") {
            Self {
                lines: vec![
                    r#"                     .                    "#,
                    r#"                    / \                   "#,
                    r#"                   /   \                  "#,
                    r#"                  /^.   \                 "#,
                    r#"                 /  .-.  \                "#,
                    r#"                /  (   ) _\               "#,
                    r#"               / _.~   ~._^\              "#,
                    r#"              /.^         ^.\             "#,
                    r#"             /               \            "#,
                    r#"            /                 \           "#,
                    r#"           /                   \          "#,
                    r#"          /                     \         "#,
                    r#"         /                       \        "#,
                    r#"        /                         \       "#,
                    r#"       /                           \      "#,
                    r#"      /                             \     "#,
                    r#"     /                               \    "#,
                    r#"    /_/                             \_\   "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "cyan".to_string(),
            }
        } else if name_lower.contains("kali") {
            Self {
                lines: vec![
                    r#"                  .......                 "#,
                    r#"                /`       `\               "#,
                    r#"               |           |              "#,
                    r#"               |   /`|\    |              "#,
                    r#"               |  / /  \   |              "#,
                    r#"               | | |    |  |              "#,
                    r#"               | | |    |  |              "#,
                    r#"               | | |   /  /               "#,
                    r#"               | | |  /  /                "#,
                    r#"               | | | /  /                 "#,
                    r#"               | | |/  /                  "#,
                    r#"               | \    /                   "#,
                    r#"                \ `--`                    "#,
                    r#"                 ........                 "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "blue".to_string(),
            }
        } else if name_lower.contains("redhat") || name_lower.contains("rhel") {
            Self {
                lines: vec![
                    r#"                                          "#,
                    r#"           .----------.                   "#,
                    r#"          /            \                  "#,
                    r#"         /              \                 "#,
                    r#"        /                \                "#,
                    r#"       /                  \               "#,
                    r#"      /                    \              "#,
                    r#"     /___                ___\             "#,
                    r#"    |    ``---......----`    |            "#,
                    r#"    |                        |            "#,
                    r#"     \                      /             "#,
                    r#"      `--.              .--`              "#,
                    r#"          \____________/                  "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "red".to_string(),
            }
        } else if name_lower.contains("cachyos") {
            Self {
                lines: vec![
                    r#"               .----------------------:   "#,
                    r#"             .+=======================.   "#,
                    r#"            :++===++==================-   "#,
                    r#"     :++-  :*++====+++++==            "#,
                    r#"     .==: -*+++=====+***++====        "#,
                    r#"         =*++++========-------        "#,
                    r#"        =*+++++=====-                 "#,
                    r#"      .+*+++++=-===:                  "#,
                    r#"     :++++=====-==:                   "#,
                    r#"    :++========-=.                    "#,
                    r#"   .+==========-.                     "#,
                    r#"   :+++++++====-                      "#,
                    r#"    :++==========.                    "#,
                    r#"     .-===========.                   "#,
                    r#"      .-===========:                  "#,
                    r#"        -=======++++::::::::::.....   "#,
                    r#"         :======++++====+++***********"#,
                    r#"          :=====+++=================="#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "cyan".to_string(),
            }
        } else if name_lower.contains("debian") {
            Self {
                lines: vec![
                    r#"         _,met$$$$$gg.                    "#,
                    r#"      ,g$$$$$$$$$$$$$$$P.                 "#,
                    r#"    ,g$$P"     """Y$$.".                  "#,
                    r#"   ,$$P'              `$$$.               "#,
                    r#"  ',$$P       ,ggs.     `$$b:             "#,
                    r#"  `d$$'     ,$P"'   .    $$$              "#,
                    r#"   $$P      d$'     ,    $$P              "#,
                    r#"   $$:      $$.   -    ,d$$'              "#,
                    r#"   $$;      Y$b._   _,d$P'                "#,
                    r#"   Y$$.    `.`"Y$$$$P"'                   "#,
                    r#"   `$$b      "-.__                        "#,
                    r#"    `Y$$                                  "#,
                    r#"     `Y$$.                                "#,
                    r#"       `$$b.                              "#,
                    r#"         `Y$$b.                           "#,
                    r#"             `"Y$b._                      "#,
                    r#"                 `"""                     "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "red".to_string(),
            }
        } else if name_lower.contains("fedora") {
            Self {
                lines: vec![
                    r#"             .',;::::;,'.                 "#,
                    r#"         .';:cccccccccccc:;,.             "#,
                    r#"      .;cccccccccccccccccccccc;.          "#,
                    r#"    .:cccccccccccccccccccccccccc:.        "#,
                    r#"  .;ccccccccccccc;.:dddl:.;ccccccc;.      "#,
                    r#" .:ccccccccccccc;OWMKOOXMWd;ccccccc:.     "#,
                    r#".:ccccccccccccc;KMMc;cc;xMMc:ccccccc:.    "#,
                    r#",cccccccccccccc;MMM.;cc;;WW::cccccccc,    "#,
                    r#":cccccccccccccc;MMM.;cccccccccccccccc:    "#,
                    r#":ccccccc;oxOOOo;MMM0OOk.;cccccccccccc:    "#,
                    r#"cccccc:0MMKxdd:;MMMkddc.;cccccccccccc;    "#,
                    r#".ccccc:XM0';cccc;MMM.;cccccccccccccc:'    "#,
                    r#" .cccc:oWMMKOkc;MMM.;cccccccccccccc:.     "#,
                    r#"  .ccccc:;cc;KMMc;cccccccccccccccc;.      "#,
                    r#"    .:ccccccccccccc;OWMKOOXMWd;cc:.       "#,
                    r#"      .;cccccccccccccccccccccc;.          "#,
                    r#"         .';:cccccccccccc:;,.             "#,
                    r#"             .',;::::;,'.                 "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "blue".to_string(),
            }
        } else if name_lower.contains("mint") {
            Self {
                lines: vec![
                    r#"             ...-:::::-...                "#,
                    r#"          .-MMMMMMMMMMMMMMM-.             "#,
                    r#"      .-MMMM`..-:::::::-..`MMMM-.         "#,
                    r#"    .:MMMM.:MMMMMMMMMMMMMMM:.MMMM:.       "#,
                    r#"   -MMM-M---MMMMMMMMMMMMMMMMMMM.MMM-      "#,
                    r#" `:MMM:MM`  :MMMM:....::-...-MMMM:MMM:`   "#,
                    r#" :MMM:MMM`  :MM:`  ``    ``  `:MMM:MMM:   "#,
                    r#".MMM.MMMM`  :MM.  -MM.  .MM-  `MMMM.MMM.  "#,
                    r#":MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:  "#,
                    r#":MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM:MMM:  "#,
                    r#":MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:  "#,
                    r#".MMM.MMMM`  :MM:--:MM:--:MM:  `MMMM.MMM.  "#,
                    r#" :MMM:MMM-  `-MMMMMMMMMMMM-`  -MMM-MMM:   "#,
                    r#"  :MMM:MMM:                :MMM:MMM:      "#,
                    r#"   .MMM.MMMM:--------------:MMMM.MMM.     "#,
                    r#"     .-MMMM.-MMMMMMMMMMMMMMM-.MMMM-.      "#,
                    r#"        .-MMMM``..-:::::-..``MMMM-.       "#,
                    r#"            .-MMMMMMMMMMMMMMM-.           "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "green".to_string(),
            }
        } else if name_lower.contains("suse") || name_lower.contains("opensuse") {
            Self {
                lines: vec![
                    r#"                  .......                 "#,
                    r#"               .-^       ^-.              "#,
                    r#"              /             \             "#,
                    r#"             /___         ___\            "#,
                    r#"             |   \       /   |            "#,
                    r#"             |    \     /    |            "#,
                    r#"             |     \   /     |            "#,
                    r#"              \     \ /     /             "#,
                    r#"               `-.   V   .-`              "#,
                    r#"                  `-...-`                 "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "green".to_string(),
            }
        } else if name_lower.contains("alpine") {
            Self {
                lines: vec![
                    r#"                 /\                       "#,
                    r#"                /  \                      "#,
                    r#"               /    \                     "#,
                    r#"              /      \                    "#,
                    r#"             /   /\   \                   "#,
                    r#"            /   /  \   \                  "#,
                    r#"           /   /    \   \                 "#,
                    r#"          /   /      \   \                "#,
                    r#"         /   /   /\   \   \               "#,
                    r#"        /   /   /  \   \   \              "#,
                    r#"       /   /   /    \   \   \             "#,
                    r#"      /   /   /      \   \   \            "#,
                    r#"     /   /   /        \   \   \           "#,
                    r#"    /_  /   /          \   \  _\          "#,
                    r#"      \/_ _/            \_ _\/            "#,
                    r#"                                          "#,
                    r#"                                          "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "blue".to_string(),
            }
        } else if name_lower.contains("nixos") {
            Self {
                lines: vec![
                    r#"            \\\\  \\\\\\                  "#,
                    r#"            \\\\  \\\\\\                  "#,
                    r#"           \\\\    \\\\\\                 "#,
                    r#"           \\\\    \\\\\\                 "#,
                    r#"          \\\\      \\\\\\                "#,
                    r#"          \\\\      \\\\\\                "#,
                    r#"          \\\\      \\\\\\                "#,
                    r#"          \\\\//////\\\\\\                "#,
                    r#"          \\\\//////\\\\\\                "#,
                    r#"          \\\\      \\\\\\                "#,
                    r#"          \\\\      \\\\\\                "#,
                    r#"           \\\\    \\\\\\                 "#,
                    r#"           \\\\    \\\\\\                 "#,
                    r#"            \\\\  \\\\\\                  "#,
                    r#"            \\\\  \\\\\\                  "#,
                    r#"                                          "#,
                    r#"                                          "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "blue".to_string(),
            }
        } else if name_lower.contains("gentoo") {
            Self {
                lines: vec![
                    r#"           -/oyddmdhs+:.                  "#,
                    r#"       -odNMMMMMMMMNNmhy+-`               "#,
                    r#"     -yNMMMMMMMMMMMNNNmmdhy+-             "#,
                    r#"   `omMMMMMMMMMMMMNmdmmmmddhhy/`          "#,
                    r#"   omMMMMMMMMMMMNhhyyyohmdddhhhdo`        "#,
                    r#"  .ydMMMMMMMMMMdhs++so/smdddhhhhdm+`      "#,
                    r#"   oyhdmNMMMMMMMNdyooydmddddhhhhyhNd.     "#,
                    r#"    :oyhhdNNMMMMMMMNNNmdhhhhhddhhyymMh    "#,
                    r#"      .:+sydNMMMMMNNNmmmdddhhhhhhmMmy     "#,
                    r#"         /mMMMMMMNNNmmmdddhhhhhmMNhs:     "#,
                    r#"        `oNMMMMMMMNNNmmmddddNmNmhs+`      "#,
                    r#"          `+ymMMMMMMNmdddddmdhhhsoo       "#,
                    r#"             .shNMMMMNmdhhhss+:::-        "#,
                    r#"               `-oydNNmhhhhys+:           "#,
                    r#"                   `-/ooyhhs/-            "#,
                    r#"                       `...               "#,
                    r#"                                          "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "magenta".to_string(),
            }
        } else if name_lower.contains("void") {
            Self {
                lines: vec![
                    r#"                  ___.                    "#,
                    r#"                 /   |                    "#,
                    r#"                /    |                    "#,
                    r#"               /     |                    "#,
                    r#"              /______|                    "#,
                    r#"              |      |                    "#,
                    r#"              |      |                    "#,
                    r#"              |      |                    "#,
                    r#"              |      |                    "#,
                    r#"              |      |                    "#,
                    r#"              |      |                    "#,
                    r#"              |      |                    "#,
                    r#"              |______|                    "#,
                    r#"               \     |                    "#,
                    r#"                \    |                    "#,
                    r#"                 \___|                    "#,
                    r#"                                          "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "green".to_string(),
            }
        } else if name_lower.contains("manjaro") {
            Self {
                lines: vec![
                    r#"    ██████████████████  ████████          "#,
                    r#"    ██████████████████  ████████          "#,
                    r#"    ██████████████████  ████████          "#,
                    r#"    ██████████████████  ████████          "#,
                    r#"    ████████            ████████          "#,
                    r#"    ████████  ████████  ████████          "#,
                    r#"    ████████  ████████  ████████          "#,
                    r#"    ████████  ████████  ████████          "#,
                    r#"    ████████  ████████  ████████          "#,
                    r#"    ████████  ████████  ████████          "#,
                    r#"    ████████  ████████  ████████          "#,
                    r#"    ████████  ████████  ████████          "#,
                    r#"    ████████  ████████  ████████          "#,
                    r#"    ████████  ████████  ████████          "#,
                    r#"    ████████  ████████  ████████          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "green".to_string(),
            }
        } else if name_lower.contains("mac") || name_lower.contains("darwin") {
            Self {
                lines: vec![
                    r#"                    'c.                   "#,
                    r#"                 ,xNMM.                   "#,
                    r#"               .OMMMMo                    "#,
                    r#"               OMMM0,                     "#,
                    r#"     .;teoxl. 'MMM0;..ccccccccccccc;.     "#,
                    r#"    oWMMMMMMXl:MMMo.,::cccccccccccccc.    "#,
                    r#"  .0MMMMMMMMMMMMMMl ccccccccccccccccc.    "#,
                    r#"  kMMMMMMMMMMMMMMMc ccccccccccccccccc.    "#,
                    r#"  WMMMMMMMMMMMMMMMc :cccccccccccccccc.    "#,
                    r#"  0MMMMMMMMMMMMMMNc .cccccccccccccccc.    "#,
                    r#"  .kMMMMMMMMMMMMMM0. .cccccccccccccc.     "#,
                    r#"    dMMMMMMMMMMMMMMK. .cccccccccccc.      "#,
                    r#"      l0WMMMMMMMMMMM:  .cccccccccc.       "#,
                    r#"         .lk0KXXXX0d.   ...........       "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "white".to_string(),
            }
        } else if name_lower.contains("windows") {
            Self {
                lines: vec![
                    r#"                                  ..,     "#,
                    r#"                      ....,,:;+ccllll     "#,
                    r#"        ...,,+:;  cllllllllllllllllll     "#,
                    r#"  ,cclllllllllll  lllllllllllllllllll     "#,
                    r#"  llllllllllllll  lllllllllllllllllll     "#,
                    r#"  llllllllllllll  lllllllllllllllllll     "#,
                    r#"  llllllllllllll  lllllllllllllllllll     "#,
                    r#"  llllllllllllll  lllllllllllllllllll     "#,
                    r#"                                          "#,
                    r#"  llllllllllllll  lllllllllllllllllll     "#,
                    r#"  llllllllllllll  lllllllllllllllllll     "#,
                    r#"  llllllllllllll  lllllllllllllllllll     "#,
                    r#"  llllllllllllll  lllllllllllllllllll     "#,
                    r#"  llllllllllllll  lllllllllllllllllll     "#,
                    r#"  `'ccllllllllll  lllllllllllllllllll     "#,
                    r#"         `' \\*::  :ccllllllllllllllll    "#,
                    r#"                       ````''*::cllll     "#,
                    r#"                                 ````     "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "blue".to_string(),
            }
        } else if name_lower.contains("ubuntu") {
            Self {
                lines: vec![
                    r#"                              ..          "#,
                    r#"         .-/+oossssoo+/-.               . "#,
                    r#"     `:+ssssssssssssssssss+:`           . "#,
                    r#"   -+ssssssssssssssssssyyssss+-         . "#,
                    r#" .ossssssssssssssssssdMMMNysssso.       . "#,
                    r#"/ssssssssssshdmmNNmmyNMMMMhssssss\      . "#,
                    r#"+ssssssssshmydMMMMMMMNddddysssssss+     . "#,
                    r#"/sssssssshNMMMyhhyyyyhmNMMMNhssssss/    . "#,
                    r#".osssssssssdMMMNytsssssyNMMMdssssso.    . "#,
                    r#"  -+sssssssssssssssssyyyssss+-          . "#,
                    r#"    `:+ssssssssssssssssss+:`            . "#,
                    r#"         .-/+oossssoo+/-.               . "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                    r#"                                          "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "yellow".to_string(),
            }
        } else {
            // Default Tux
            Self {
                lines: vec![
                    r#"                  .......                 "#,
                    r#"                 /--.-.--\                "#,
                    r#"                 | o | o |                "#,
                    r#"                 |.-...-.|                "#,
                    r#"                _/-.___.-\_               "#,
                    r#"               /           \              "#,
                    r#"             /               \            "#,
                    r#"            |  ^           ^  |           "#,
                    r#"            | |             | |           "#,
                    r#"            | |             | |           "#,
                    r#"            | |             | |           "#,
                    r#"            \  \           /  /           "#,
                    r#"            /   `---------`   \           "#,
                    r#"           /___/_       _\_____\          "#,
                    r#"                 `-----`                  "#,
                    r#"                                          "#,
                    r#"                                          "#,
                ].iter().map(|s| s.to_string()).collect(),
                color: "blue".to_string(),
            }
        }
    }
}
