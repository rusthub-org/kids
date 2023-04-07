let first = params[0];
let second = params[1];

switch params.len {
    3 => {
        let opr = params[2];

        switch opr {
            "lt" => first < second,
            "lte" => first <= second,
            "gt" => first > second,
            "gte" => first >= second,
            "ct" => first.contains(second),
            _ => "操作符错误！",
        }
    },
    2 => first == second,
    _ => "参数错误！",
}
