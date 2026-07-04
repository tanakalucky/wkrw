use std::collections::HashMap;

const WA: [&str; 15] = [
    r#"   #          "#,
    r#"   #          "#,
    r#"   #          "#,
    r#"   #   #####  "#,
    r#"####  ##   ## "#,
    r#"   # ##     ##"#,
    r#"   ###       #"#,
    r#"   ##        #"#,
    r#"   #         #"#,
    r#"  ##         #"#,
    r#" ###         #"#,
    r#"## #        ##"#,
    r#"   #       ## "#,
    r#"   #     ###  "#,
    r#"   #          "#,
];

const KA: [&str; 15] = [
    r#"    #          "#,
    r#"    #          "#,
    r#"    #      #   "#,
    r#"    #      ##  "#,
    r#"########    ## "#,
    r#"    #   #    # "#,
    r#"    #   #    ##"#,
    r#"   ##   #     #"#,
    r#"   #    #     #"#,
    r#"   #    #      "#,
    r#"  ##    #      "#,
    r#"  #     #      "#,
    r#" ##    ##      "#,
    r#"##     #       "#,
    r#"     ###       "#,
];

const RU: [&str; 15] = [
    r#"            "#,
    r#" #########  "#,
    r#"       ##   "#,
    r#"      ##    "#,
    r#"     ##     "#,
    r#"    ##      "#,
    r#"   #######  "#,
    r#"  ##     ## "#,
    r#" ##       ##"#,
    r#"##         #"#,
    r#"   ###     #"#,
    r#"  #  ##    #"#,
    r#"  #   #   ##"#,
    r#"  ##  #  ## "#,
    r#"   #######  "#,
];

const STR_GAP: [&str; 15] = [
    r#"  "#, r#"  "#, r#"  "#, r#"  "#, r#"  "#, r#"  "#, r#"  "#, r#"  "#, r#"  "#, r#"  "#,
    r#"  "#, r#"  "#, r#"  "#, r#"  "#, r#"  "#,
];

pub fn get_char_map() -> HashMap<char, [&'static str; 15]> {
    let mut map = HashMap::new();

    map.insert('わ', WA);
    map.insert('か', KA);
    map.insert('る', RU);

    map
}

pub fn generate_ascii_art(text: &str, char_map: &HashMap<char, [&'static str; 15]>) -> Vec<String> {
    let mut art = vec![String::new(); 15];
    let mut first = true;

    for c in text.chars() {
        if let Some(lines) = char_map.get(&c) {
            for i in 0..15 {
                if !first {
                    art[i].push_str(STR_GAP[i]);
                }
                art[i].push_str(lines[i]);
            }
            first = false;
        }
    }

    art
}
