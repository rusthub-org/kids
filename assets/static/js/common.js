"use strict";

String.prototype.extension = function () {
    let ext = null;

    let name = this.toLowerCase();
    let i = name.lastIndexOf(".");
    if (i > -1) {
        ext = name.substring(i);
    }

    return ext;
}

Array.prototype.contain = function (obj) {
    for (let i = 0; i < this.length; i++) {
        if (this[i] === obj)
            return true;
    }

    return false;
};

function setInputByCheckbox(input_name, checkbox) {
    let input_obj = $("input[name=" + input_name + "]");

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
