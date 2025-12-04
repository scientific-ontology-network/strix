import {Component, EventEmitter, Input, Output} from '@angular/core';
import {CommonModule} from '@angular/common';
import {ClassExpressionView} from '../../bindings/ClassExpressionView';
import {ObjectPropertyExpressionView} from '../../bindings/ObjectPropertyExpressionView';
import {AnnotationValueView} from "../../bindings/AnnotationValueView";
import {addLineBreaks, LiteralComponent} from "../literal";

export function getUntaggedStringFromAnnotationValue(value: AnnotationValueView): string {
    if (typeof value === "string") {
        return value;
    } else {
        return value.value;
    }
}

@Component({
    selector: 'app-annotation-value',
    standalone: true,
    imports: [CommonModule, LiteralComponent],
    template: `
        @if (typeof this.expression === "string") {
               <div>\`{{addLineBreaks(this.expression)}}\`</div>
        } @else
        {
                <app-literal
                    [expression]="this.expression!"
                    [labelMap]="labelMap"
                />
        }`
})
export class AnnotationValueComponent {
    @Input() expression!: AnnotationValueView;
    @Input() labelMap!: Map<String, AnnotationValueView>;
    protected readonly addLineBreaks = addLineBreaks;
}
