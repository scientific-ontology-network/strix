import {Component, EventEmitter, Input, Output} from '@angular/core';
import {CommonModule} from '@angular/common';
import {ClassExpressionView} from '../../bindings/ClassExpressionView';
import {ObjectPropertyExpressionView} from '../../bindings/ObjectPropertyExpressionView';

@Component({
    selector: 'app-object-property-expression',
    standalone: true,
    imports: [CommonModule],
    template: `
        @switch (this.expression.type) {
            @case ('ObjectProperty') {
                <a href="#" (click)="onIriClick.emit(this.expression.iri)">\`{{ labelMap.get(this.expression.iri!) ?? this.expression.iri }}\`</a>
            }
            @case ('InverseObjectProperty') {
                inverse
                <app-object-property-expression
                        [expression]="{ type: 'ObjectProperty', iri: this.expression.property }"
                        (onIriClick)="onIriClick.emit($event)">
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
    @Input() labelMap!: Map<String, String>;
    @Output() onIriClick: EventEmitter<string> = new EventEmitter();
}

@Component({
    selector: 'app-class-expression',
    standalone: true,
    imports: [CommonModule, ObjectPropertyExpressionComponent],
    template: `
        @switch (this.expression.type) {
            @case ('Class') {
                <a href="#" (click)="onIriClick.emit(this.expression.iri)">\`{{ this.labelMap.get(this.expression.iri!) ?? this.expression.iri }}\`</a>
            }
            @case ('ObjectSomeValuesFrom') {
                <app-object-property-expression
                    [expression]="this.expression.property!"
                    [labelMap]="labelMap"
                    (onIriClick)="onIriClick.emit($event)"
                />
                <strong  class="px-1"> some </strong>
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onIriClick)="onIriClick.emit($event)"></app-class-expression>
            }
            @case ('ObjectAllValuesFrom') {
                <app-object-property-expression
                        [expression]="this.expression.property!"
                        [labelMap]="this.labelMap"
                        (onIriClick)="onIriClick.emit($event)"
                /> 
                <strong class="px-1"> only </strong>
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onIriClick)="onIriClick.emit($event)"></app-class-expression>
            }
            @case ('ObjectIntersectionOf') {
                <span *ngFor="let op of this.expression.operands; let last = last">
          <app-class-expression [expression]="op" [labelMap]="this.labelMap" (onIriClick)="onIriClick.emit($event)"></app-class-expression>
          <strong class="px-1" *ngIf="!last" >and</strong>
        </span>
            }
            @case ('ObjectUnionOf') {
                <span *ngFor="let op of this.expression.operands; let last = last">
          <app-class-expression [expression]="op" [labelMap]="this.labelMap" (onIriClick)="onIriClick.emit($event)"></app-class-expression>
          <strong class="px-1" *ngIf="!last">or</strong>
        </span>
            }
            @case ('ObjectComplementOf') {
                not
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onIriClick)="onIriClick.emit($event)"></app-class-expression>
            }
            @case ('ObjectMinCardinality') {
                <app-object-property-expression
                        [expression]="this.expression.property!"
                        [labelMap]="labelMap"
                        (onIriClick)="onIriClick.emit($event)"
                /> min {{ this.expression.cardinality }}
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onIriClick)="onIriClick.emit($event)"></app-class-expression>
            }
            @case ('ObjectMaxCardinality') {
                <app-object-property-expression
                        [expression]="this.expression.property!"
                        [labelMap]="labelMap"
                        (onIriClick)="onIriClick.emit($event)"
                /> max {{ this.expression.cardinality }}
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onIriClick)="onIriClick.emit($event)"></app-class-expression>
            }
            @case ('ObjectExactCardinality') {
                <app-object-property-expression
                        [expression]="this.expression.property!"
                        [labelMap]="labelMap"
                        (onIriClick)="onIriClick.emit($event)"
                /> exactly {{ this.expression.cardinality }}
                <app-class-expression [expression]="this.expression.class_expression"
                                      [labelMap]="this.labelMap"
                                      (onIriClick)="onIriClick.emit($event)"></app-class-expression>
            }
            @case ('ObjectOneOf') {
                {{ this.expression.individuals.join(' or ') }}
            }
            @case ('ObjectHasValue') {
                <app-object-property-expression
                        [expression]="this.expression.property!"
                        [labelMap]="labelMap"
                        (onIriClick)="onIriClick.emit($event)"
                /> value {{ this.expression.individual }}
            }
            @default {
                {{console.log(this.expression)}}
                <span>Unknown expression type</span>
            }
        }
    `
})
export class ClassExpressionComponent {
    @Input() expression!: ClassExpressionView;
    @Input() labelMap!: Map<String, String>;
    @Output() onIriClick: EventEmitter<string> = new EventEmitter();
    protected readonly console = console;
}
