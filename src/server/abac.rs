use std::collections::HashMap;

#[derive(Default)]
pub struct AbacAttrList {
    v0: String,
    v1: String,
    v2: String,
    v3: String,
    v4: String,
    v5: String,
    v6: String,
    v7: String,
    v8: String,
    v9: String,
    v10: String,
    pub name_map: HashMap<String, String>,
}

pub fn to_upper_first_char(s: String) -> String {
    format!("{}{}", &s[..1].to_uppercase(), &s[1..])
}
// TRY TO CONVERT INTO VECTOR OF STRING
pub fn resolve_abac(s: String) -> Result<AbacAttrList, &'static str> {
    // THIS MAP IS EMPTY
    let json_map: HashMap<String, String> = HashMap::new();
    let mut attr_list: AbacAttrList = AbacAttrList::default();
    attr_list.name_map = HashMap::new();
    // need to update json_map based on s
    let mut i: usize = 0;
    for (k, v) in json_map.iter() {
        let key: String = to_upper_first_char(k.to_string());
        let value: String = format!("{}{}", "v", &(i.to_string()));
        // attr_list is empty
        // attr_list.name_map[key] = "V" + strconv.Itoa(i);

        match i {
            0 => attr_list.v0 = value,
            1 => attr_list.v1 = value,
            2 => attr_list.v2 = value,
            3 => attr_list.v3 = value,
            4 => attr_list.v4 = value,
            5 => attr_list.v5 = value,
            6 => attr_list.v6 = value,
            7 => attr_list.v7 = value,
            8 => attr_list.v8 = value,
            9 => attr_list.v9 = value,
            10 => attr_list.v10 = value,
            _ => (),
        }
        i = i + 1;
    }
    Ok(attr_list)
}
