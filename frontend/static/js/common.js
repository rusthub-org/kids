"use strict";

function setInputByCheckbox(input_name, checkbox) {
    var input_obj = $("input[name=" + input_name + "]");

    if (checkbox instanceof Object) {
        input_obj.val($(checkbox).is(':checked'));
    }
    else {
        input_obj.val($("#" + checkbox).is(':checked'));
    }
}

function str2arr(str) {
    let arr = [];
    let str_trim = str.trim();
    if (undefined != str_trim && "" !== str_trim) {
        arr = str.split(",");
    }

    return arr;
}
