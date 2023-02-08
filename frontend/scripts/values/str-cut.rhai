let input_str = params[0];

input_str.remove("\r");
input_str.remove("\n");
input_str.remove("\t");
input_str.remove("#");
input_str.remove("*");
input_str.trim();

switch params.len {
    1 => input_str,
    2 => {
        let len_trc = params[1];
    
        input_str.truncate(len_trc);
        input_str
    },
    3 => {
        let len_start = params[1];
        let len_end = params[2];
        
        input_str.sub_string(len_start, len_end)
    },
    _ => "参数错误！",
}
