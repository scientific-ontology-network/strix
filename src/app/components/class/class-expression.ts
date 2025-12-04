import {Component, EventEmitter, Input, Output} from '@angular/core';
import {CommonModule} from '@angular/common';
import {ClassExpressionView} from '../../bindings/ClassExpressionView';
import {ObjectPropertyExpressionView} from '../../bindings/ObjectPropertyExpressionView';
import {AnnotationValueComponent} from "../annotation/annotation-value";
import {AnnotationView} from "../../bindings/AnnotationView";
import {AnnotationValueView} from "../../bindings/AnnotationValueView";
import {ObjectPropertyExpressionComponent} from "../property/object_property";


@Component({
    selector: 'app-class-expression',
    standalone: true,
    imports: [CommonModule, ObjectPropertyExpressionComponent, AnnotationValueComponent, AnnotationValueComponent],
    template: `
        @switch (this.expression.type) {
            @case ('Class') {
                <a href="#" (click)="onClassClick.emit(this.expression.iri)">
                    <app-annotation-value 
                        [expression]="labelMap.get(this.expression.iri) ?? this.expression.iri"
                        [labelMap]="labelMap"/>
                </a>
            }
            @case ('ObjectSomeValuesFrom') {
                <app-object-property-expression
                    [expression]="this.expression.property!"
                    [labelMap]="labelMap"
                    (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"
                />
                <strong  class="px-1"> some </strong>
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onClassClick)="onClassClick.emit($event)"
                                      (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"></app-class-expression>
            }
            @case ('ObjectAllValuesFrom') {
                <app-object-property-expression
                        [expression]="this.expression.property!"
                        [labelMap]="this.labelMap"
                        (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"
                /> 
                <strong class="px-1"> only </strong>
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onClassClick)="onClassClick.emit($event)"
                                      (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"></app-class-expression>
            }
            @case ('ObjectIntersectionOf') {
                <span *ngFor="let op of this.expression.operands; let last = last">
          <app-class-expression 
                  [expression]="op" 
                  [labelMap]="this.labelMap"
                  (onClassClick)="onClassClick.emit($event)"
                  (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"></app-class-expression>
          <strong class="px-1" *ngIf="!last" >and</strong>
        </span>
            }
            @case ('ObjectUnionOf') {
                <span *ngFor="let op of this.expression.operands; let last = last">
          <app-class-expression 
                  [expression]="op" 
                  [labelMap]="this.labelMap"
                  (onClassClick)="onClassClick.emit($event)"
                  (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"></app-class-expression>
          <strong class="px-1" *ngIf="!last">or</strong>
        </span>
            }
            @case ('ObjectComplementOf') {
                not
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onClassClick)="onClassClick.emit($event)"
                                      (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"></app-class-expression>
            }
            @case ('ObjectMinCardinality') {
                <app-object-property-expression
                        [expression]="this.expression.property!"
                        [labelMap]="labelMap"
                        (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"
                /> min {{ this.expression.cardinality }}
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onClassClick)="onClassClick.emit($event)"
                                      (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"></app-class-expression>
            }
            @case ('ObjectMaxCardinality') {
                <app-object-property-expression
                        [expression]="this.expression.property!"
                        [labelMap]="labelMap"
                        (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"
                /> max {{ this.expression.cardinality }}
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onClassClick)="onClassClick.emit($event)"
                                      (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"></app-class-expression>
            }
            @case ('ObjectExactCardinality') {
                <app-object-property-expression
                        [expression]="this.expression.property!"
                        [labelMap]="labelMap"
                        (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"
                /> exactly {{ this.expression.cardinality }}
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onClassClick)="onClassClick.emit($event)"
                                      (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"
                ></app-class-expression>
            }
            @case ('ObjectOneOf') {
                {{ this.expression.individuals.join(' or ') }}
            }
            @case ('ObjectHasValue') {
                <app-object-property-expression
                        [expression]="this.expression.property!"
                        [labelMap]="labelMap"
                        (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"
                /> value {{ this.expression.individual }}
            }
            @default {
                {{console.log(this.expression)}}
                <span>Unknown expression type {{this. expression}}</span>
            }
        }
    `
})
export class ClassExpressionComponent {
    @Input() expression!: ClassExpressionView;
    @Input() labelMap!: Map<String, AnnotationValueView>;
    @Output() onClassClick: EventEmitter<string> = new EventEmitter();
    @Output() onObjectPropertyClick: EventEmitter<string> = new EventEmitter();
    protected readonly console = console;
}
