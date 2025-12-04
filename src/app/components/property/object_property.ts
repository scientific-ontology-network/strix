import {Component, EventEmitter, Input, Output} from "@angular/core";
import {CommonModule} from "@angular/common";
import {ObjectPropertyExpressionView} from "../../bindings/ObjectPropertyExpressionView";
import {AnnotationValueView} from "../../bindings/AnnotationValueView";

@Component({
    selector: 'app-object-property-expression',
    standalone: true,
    imports: [CommonModule],
    template: `
        @switch (this.expression.type) {
            @case ('ObjectProperty') {
                <a href="#" (click)="onObjectPropertyClick.emit(this.expression.iri)">\`{{ labelMap.get(this.expression.iri!) ?? this.expression.iri }}\`</a>
            }
            @case ('InverseObjectProperty') {
                inverse
                <app-object-property-expression
                        [expression]="{ type: 'ObjectProperty', iri: this.expression.property }"
                        (onObjectPropertyClick)="onObjectPropertyClick.emit($event)">
                </app-object-property-expression>
            }
            @default {
                <span>Unknown property expression type</span>
            }
        }
    `
})
export class ObjectPropertyExpressionComponent {
    @Input() expression!: ObjectPropertyExpressionView;
    @Input() labelMap!: Map<String, AnnotationValueView>;
    @Output() onObjectPropertyClick: EventEmitter<string> = new EventEmitter();
}