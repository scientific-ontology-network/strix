import {Component, EventEmitter, Input, Output} from '@angular/core';
import {CommonModule} from '@angular/common';
import {ClassExpressionView} from '../../bindings/ClassExpressionView';
import {ObjectPropertyExpressionView} from '../../bindings/ObjectPropertyExpressionView';
import {AnnotationValueComponent} from "../annotation/annotation-value";
import {AnnotationView} from "../../bindings/AnnotationView";
import {AnnotationValueView} from "../../bindings/AnnotationValueView";
import {ObjectPropertyExpressionComponent} from "../property/object_property";
import {OntologySymbolView} from "../../bindings/OntologySymbolView";
import {ClassExpressionComponent} from "../class/class-expression";


@Component({
    selector: 'app-ontology-symbol',
    standalone: true,
    imports: [CommonModule, ObjectPropertyExpressionComponent, AnnotationValueComponent, AnnotationValueComponent, ClassExpressionComponent],
    template: `
        @switch (this.symbol.symbol_type) {
            @case ('Class') {
                <app-class-expression
                        [expression]="{'type':'Class','iri':this.symbol.value!}"
                        [labelMap]="labelMap"
                        (onClassClick)="onClassClick.emit($event)"
                        (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"
                />
            }
            @case ('Role') {
                <app-object-property-expression
                    [expression]="{'type':'ObjectProperty','iri':this.symbol.value!}"
                    [labelMap]="labelMap"
                    (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"
                />
            }
        }
    `
})
export class OntologySymbolComponent {
    @Input() symbol!: OntologySymbolView;
    @Input() labelMap!: Map<String, AnnotationValueView>;
    @Output() onClassClick: EventEmitter<string> = new EventEmitter();
    @Output() onObjectPropertyClick: EventEmitter<string> = new EventEmitter();
    protected readonly console = console;
}
