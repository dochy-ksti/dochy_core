use crate::imp::rust_to_json::list::default_to_json::default_to_json;
use crate::imp::structs::my_json::Value;
use crate::imp::structs::mil_def_obj::MilDefObj;

pub fn inner_mut_def_to_json(d : &MilDefObj) -> Value{
    let mut result : Vec<Value> = Vec::new();

    result.push(Value::String("MilDef".to_string()));
    // if d.compatible().len() != 0{
    //     result.push(string_set_to_json("Compatible", &btree_set(d.compatible())));
    // }

    result.push(Value::Array(vec![default_to_json(d.list_def())]));


    return Value::Array(result);
}