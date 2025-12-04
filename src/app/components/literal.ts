import {Component, EventEmitter, Input, Output} from '@angular/core';
import {CommonModule} from '@angular/common';
import {LiteralView} from "../bindings/LiteralView";
import {AnnotationValueView} from "../bindings/AnnotationValueView";

@Component({
    selector: 'app-literal',
    standalone: true,
    imports: [CommonModule],
    template: `
        @switch (this.expression.type) {
            @case ('Simple') {
                \`{{ addLineBreaks(this.expression.value) }}\`
            }
            @case ('Language') {
                \`{{this.expression.value}}\`&#64;{{this.expression.language}}
            }
            @case ('Datatype') {
                \`{{this.expression.value}}\`^^{{this.expression.datatype}}
            }
            @default {
                <span>Unknown literal type {{this. expression}}</span>
            }
        }
    `
})
export class LiteralComponent {
    @Input() expression!: LiteralView;
    @Input() labelMap!: Map<String, AnnotationValueView>;
    protected readonly addLineBreaks = addLineBreaks;
}

export function addLineBreaks(text: string){

    if(text.includes("\n")){
        return text;
    }

    var result = "";

    while(text != "") {

        var i = 80;
        while(text.charAt(i) != ' ' && text.length > i && i > 0){
            i--;
        }


        result += text.slice(0,i);
        result += "\n";
        text = text.substring(i);
    }

    return result;
}