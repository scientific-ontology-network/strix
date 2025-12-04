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
                {{this.expression.value}}
            }
            @case ('Language') {
                {{this.expression.value}}&#64;{{this.expression.language}}
            }
            @case ('Datatype') {
                {{this.expression.value}}^^{{this.expression.datatype}}
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
    protected readonly console = console;
}
