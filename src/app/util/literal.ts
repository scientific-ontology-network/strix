import {LiteralView} from "../bindings/LiteralView";


export function getEnglishLiteral(literals: LiteralView[]){
    let result = null
    for (let i=0; i<literals.length; i++) {
        let l = literals[i];
        if (l.type == "Language" && l.language == "en"){
            return l.value
        } else if (l.type == "Simple") {
            result = l.value
        }
    }
    return result
}
