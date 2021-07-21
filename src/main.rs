use prc::{ParamKind, ParamList, ParamStruct};

fn diff_param(old: &ParamKind, new: &ParamKind) -> Option<ParamKind> {
    match (old, new) {
        // Containers
        (ParamKind::List(old), ParamKind::List(new)) => {
            match diff_param_list(old, new) {
                Some(list) => Some(ParamKind::List(list)),
                None => None
            }
        },
        (ParamKind::Struct(old), ParamKind::Struct(new)) => {
            match diff_param_struct(old, new) {
                Some(pstruct) => Some(ParamKind::Struct(pstruct)),
                None => None
            }
        },

        // Literals
        (ParamKind::Bool(old), ParamKind::Bool(new)) => {
            if *old != *new {
                Some(ParamKind::Bool(*new))
            } else {
                None
            }
        },
        (ParamKind::I8(old), ParamKind::I8(new)) => {
            if *old != *new {
                Some(ParamKind::I8(*new))
            } else {
                None
            }
        },
        (ParamKind::U8(old), ParamKind::U8(new)) => {
            if *old != *new {
                Some(ParamKind::U8(*new))
            } else {
                None
            }
        },
        (ParamKind::I16(old), ParamKind::I16(new)) => {
            if *old != *new {
                Some(ParamKind::I16(*new))
            } else {
                None
            }
        },
        (ParamKind::U16(old), ParamKind::U16(new)) => {
            if *old != *new {
                Some(ParamKind::U16(*new))
            } else {
                None
            }
        },
        (ParamKind::I32(old), ParamKind::I32(new)) => {
            if *old != *new {
                Some(ParamKind::I32(*new))
            } else {
                None
            }
        },
        (ParamKind::U32(old), ParamKind::U32(new)) => {
            if *old != *new {
                Some(ParamKind::U32(*new))
            } else {
                None
            }
        },
        (ParamKind::Float(old), ParamKind::Float(new)) => {
            if *old != *new {
                Some(ParamKind::Float(*new))
            } else {
                None
            }
        },
        (ParamKind::Hash(old), ParamKind::Hash(new)) => {
            if *old != *new {
                Some(ParamKind::Hash(*new))
            } else {
                None
            }
        },
        (ParamKind::Str(old), ParamKind::Str(new)) => {
            if old != new {
                Some(ParamKind::Str(new.clone()))
            } else {
                None
            }
        },
        _ => None
    }
}

fn diff_param_list(old: &ParamList, new: &ParamList) -> Option<ParamList> {
    let ParamList(old) = old;
    let ParamList(new) = new;

    let mut diffs = Vec::new();

    let mut different: bool = false;

    for (index, param) in old.iter().enumerate() {
        if let Some(new_param) = new.get(index) {
            if let Some(diff) = diff_param(param, new_param) {
                diffs.push(diff);
                different = true;
                continue;
            }
        }
        if let ParamKind::I8(_) = param {
            diffs.push(ParamKind::Bool(false));
        } else {
            diffs.push(ParamKind::I8(0));
        }
    }

    if different {
        Some(ParamList(diffs))
    } else {
        None
    }
}

fn diff_param_struct(old: &ParamStruct, new: &ParamStruct) -> Option<ParamStruct> {
    fn search_struct(what: prc::hash40::Hash40, params: &Vec<(prc::hash40::Hash40, ParamKind)>) -> Option<&ParamKind> {
        for (hash, param) in params.iter() {
            if *hash == what {
                return Some(param);
            }
        }
        None
    }

    let ParamStruct(old) = old;
    let ParamStruct(new) = new;

    let mut diffs = Vec::new();

    for (hash, param) in old.iter() {
        if let Some(new_param) = search_struct(*hash, new) {
            if let Some(diff) = diff_param(param, new_param) {
                diffs.push((*hash, diff));
            }
        }
    }

    if !diffs.is_empty() {
        Some(ParamStruct(diffs))
    } else {
        None
    }
}

fn main() {
    static USAGE: &'static str = "Usage: parcel-differ path/to/old.prc path/to/new.prc path/to/out.prc";
    let mut args = std::env::args();
    let _ = args.next();

    let old_path = args.next().expect(USAGE);
    let new_path = args.next().expect(USAGE);
    let out_path = args.next().expect(USAGE);

    let old_struct = prc::open(old_path).expect("Unable to parse the old param file!");
    let new_struct = prc::open(new_path).expect("Unable to parse the new param file!");

    let diffed = diff_param_struct(&old_struct, &new_struct).expect("No differences found!");
    prc::save(out_path, &diffed).expect("Unable to save the diffed params!");
}
