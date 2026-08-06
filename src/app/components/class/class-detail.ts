import {
  Component,
  computed, effect,
  EventEmitter,
  Input,
  OnChanges,
  Output,
  Signal,
  signal,
  SimpleChanges
} from '@angular/core';
import { CommonModule } from '@angular/common';
import {invoke} from "@tauri-apps/api/core";
import {OntologyData} from "../../services/ontology.service";
import {ClassDetailsService} from "../../services/class.service";
import {ClassExpressionComponent} from "./class-expression";
import {LiteralView} from "../../bindings/LiteralView";
import {v4 as uuidv4} from 'uuid';
import {OntologySymbolView} from "../../bindings/OntologySymbolView";
import {AnnotationValueComponent} from "../annotation/annotation-value";
import {ObjectPropertyExpressionComponent} from "../property/object_property";
import {CdkAccordionModule} from '@angular/cdk/accordion';
import { OntologyEditorComponent } from '../../pages/ontology-editor';
import CompoundMap from '../../util/compound-map';


@Component({
  selector: 'app-class-detail',
  standalone: true,
  imports: [CommonModule, ClassExpressionComponent, ObjectPropertyExpressionComponent, AnnotationValueComponent, ObjectPropertyExpressionComponent, CdkAccordionModule],
  preserveWhitespaces: true,
  template: `
    <div class="p-6">
      @if (loading()) {
        <p>Loading...</p>
      } @else {
        <div class="flex items-start justify-between gap-4">
          <div>
            <h2 class="text-2xl font-semibold text-slate-900">
              <app-annotation-value [expression]="this.ontologyData().labels.get(iri) ?? iri" [labelMap]="ontologyData().labels" />
            </h2>
            @if(iri){            
              <p class="text-sm text-slate-500 mt-1">{{ iri }}</p>
            }
          </div>
        </div>
        @if(this.classDetails()?.definition){
            <p class="mt-4 text-slate-700">{{ this.classDetails()?.definition }}</p>
        }
        <div class="mt-6 grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <h3 class="text-sm font-medium text-slate-600 uppercase tracking-wider">Annotations</h3>
            <div class="mt-2 rounded-md border border-slate-200 divide-y">

              @if (!this.classDetails()?.annotations?.size) {
                <div class="px-3 py-2 text-sm text-slate-500">No annotations</div>
              } @else {
                @for (anno of this.classDetails()?.annotations!.entries(); track $index) {
                <div class="px-3 py-2">
                  <b><app-annotation-value 
                        [expression]="this.ontologyData().labels.get(anno[0]) ?? anno[0]" 
                        [labelMap]="ontologyData().labels"/>
                  </b>
                  
                    @for(v of anno[1]; track v) {
                    <div class="px-3 py-2">
                        <div class="text-xs text-slate-500" ngPreserveWhitespaces style="white-space: pre"><app-annotation-value
                          [expression]="v"
                          [labelMap]="ontologyData().labels" />
                        </div>
                      
                    </div>
                    }
                  
                  
                </div>
                }
              }
            </div>
            <div>
            <h3 class="text-sm font-medium text-slate-600 uppercase tracking-wider">Axioms</h3>
            <div class="mt-2 rounded-md border border-slate-200">
              @if(this.classDetails()?.equivalentTo?.length){
                <div class="border-t px-3 py-2 text-sm"><span
                    class="text-slate-500">Equivalent To:</span>
                  <span class="ml-2 inline-flex gap-1 flex-wrap">
                    <ul>
                    @for (p of this.classDetails()?.equivalentTo; track $index) {
                      <li><app-class-expression [expression]="p" [labelMap]="ontologyData().labels"
                                                (onClassClick)="onClassClick.emit($event)"
                                                (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"/></li>
                    }
                    </ul>
                  </span>
                </div>
              }
              @if(this.classDetails()?.superclasses?.length){
                <div class="border-t px-3 py-2 text-sm"><span
                    class="text-slate-500">SubClass Of:</span>
                  <span class="ml-2 inline-flex gap-1 flex-wrap">
                    <ul>
                    @for (p of this.classDetails()?.superclasses; track $index) {
                      <li><app-class-expression [expression]="p" [labelMap]="ontologyData().labels"
                                                (onClassClick)="onClassClick.emit($event)"
                                                (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"/></li>
                    }
                    </ul>
                  </span>
                </div>
              }
              
            </div>

          </div>

          </div>
        
        
        <div>
          <span class="text-sm font-medium text-slate-600 uppercase tracking-wider">Depends On:</span> <br>
          <div class="px-3 py-2 text-sm" *ngIf="this.classDetails()?.dependsOn?.length">

            @let depDiffMissing = (this.left_not_right().size == 0 && this.right_not_left().size == 0);
            
            @if(depDiffMissing){
                <span class="ml-3 text-sm font-medium text-gray-400"> Upload a different version of your ontology to view dependency differences.</span>
              } @else {
                <button title="Copy" class="px-1 py-1 absolute right-2 top-2">
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#D4D4D8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-copy">
                    <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
                    <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
                  </svg>
                </button>
                <label class="relative inline-flex cursor-pointer items-center">
                  <div class="h-6"> </div>
                  <span class="ml-3 text-sm font-medium text-gray-400"> Show only original dependencies </span>
                  <div class="w-2"> </div>
                </label>
                <label class="relative inline-flex cursor-pointer items-center">
                  <input type="checkbox" value="" class="peer sr-only" (change)="onDepChange()" checked/>
                  <div class="peer h-6 w-11 rounded-full bg-gray-300 after:absolute after:left-[2px] after:top-[2px] after:h-5 after:w-5 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-400 peer-checked:after:translate-x-full peer-checked:after:border-white "></div>
                  <span class="ml-3 text-sm font-medium text-gray-400"> Compare with other ontology </span>
                </label>
              }

            

            @if(this.displayAllDeps || depDiffMissing){

              <cdk-accordion class="accordion">
                @for (dependency of this.classDetails()?.dependsOn; track $index){  
                  <cdk-accordion-item #ClassAccordionItem ="cdkAccordionItem" (opened)="depDetails(dependency.value)">
                    @switch (dependency.symbol_type) {
                      @case('Class') {
                        <div class="mt-2 rounded-md border border-slate-200">
                          <div class="px-3 py-2 text-sm" style="font-weight: 600">
                            <app-class-expression [expression]="{'type':'Class','iri':dependency.value}" [labelMap]="ontologyData().labels"
                                                  (onClassClick)="onClassClick.emit($event)"/>  
                            <span class="toggle" (click)="ClassAccordionItem.toggle()" style="float: right">
                              {{ ClassAccordionItem.expanded ? '∧' : '∨' }}
                            </span>                                                     
                          </div>
                          <div class="accordion-item-body" role="region" [style.display]="ClassAccordionItem.expanded ? '' : 'none'">
                            <div class="px-3 py-2 text-sm text-slate-500"> 
                              <span class="text-sm font-medium text-slate-700 ">
                                {{"Definition:"}}
                              </span>
                              <br> 
                              {{this.dependencyDetails()?.definition}}
                            </div>
                            <div class="px-3 py-2 ext-sm text-slate-500">
                              <span class="text-sm font-medium text-slate-700 ">
                              {{"Axioms: "}} 
                              </span>
                              <br>
                              @for (p of this.dependencyDetails()?.superclasses; track $index) {
                                <span class="text-sm font-medium">{{"SubclassOf "}}</span>
                                <app-class-expression [expression]="p" [labelMap]="ontologyData().labels"/><br>
                              }
                              @for (p of this.dependencyDetails()?.equivalentTo; track $index) {
                                <span class="text-sm font-medium"> {{"EquivalentTo "}} </span>
                                <app-class-expression [expression]="p" [labelMap]="ontologyData().labels"/><br>
                              }
                            </div>
                          </div>
                        </div>
                      } @case('Role') {
                        <div class="mt-2 rounded-md border border-slate-200">        
                          <div class="px-3 py-2" style="font-weight: 600">
                            <app-object-property-expression [expression]="{'type':'ObjectProperty','iri':dependency.value}" [labelMap]="ontologyData().labels"
                                                            (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"/>     
                            <span class="toggle" (click)="ClassAccordionItem.toggle()" style="float: right">
                              {{ ClassAccordionItem.expanded ? '∧' : '∨' }}
                            </span>                         
                          </div>
                          <div class="accordion-item-body" role="region" [style.display]="ClassAccordionItem.expanded ? '' : 'none'">
                            <div class="px-3 py-2"> 
                              {{"Details on object properties are not currently available."}} 
                            </div>
                          </div>                          
                        </div>
                      }    
                    }
                  </cdk-accordion-item>
                }
              </cdk-accordion>

            } @else {

              <cdk-accordion class="accordion">

                <cdk-accordion-item #accordionItem="cdkAccordionItem">
                  <div class="mt-2 rounded-md border border-slate-200">
                    @let hasAddedDeps = this.left_not_right().get({symbol_type: 'Class', value: iri});
                    <div class="px-3 py-2">
                      <b [class.text-slate-500]="!hasAddedDeps">{{"Added Dependencies"}}</b>
                      <span [class.text-slate-500]="!hasAddedDeps" class="toggle" style="float: right" (click)="accordionItem.toggle()">
                        Click to {{ accordionItem.expanded ? 'close' : 'open' }}
                      </span>
                  </div>
                  <div class="accordion-item-body" role="region" [style.display]="accordionItem.expanded ? '' : 'none'">
                    <div class="px-3 py-2">
                      <cdk-accordion class="class-accordion">
                        @if(hasAddedDeps){
                          @for (dep of hasAddedDeps; track $index) {
                            <cdk-accordion-item #ClassAccordionItem ="cdkAccordionItem" (opened)="depDetails(dep.value)">
                              <div class="mt-2 rounded-md border border-slate-200">
                                <div class="px-3 py-2"  style="font-weight: 600">
                                  <app-class-expression [expression]="{'type':'Class','iri':dep.value}" [labelMap]="ontologyData().labels"
                                                        (onClassClick)="onClassClick.emit($event)"
                                                        (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"/>    
                                  <span class="toggle" (click)="ClassAccordionItem.toggle()" style="float: right">
                                    {{ ClassAccordionItem.expanded ? '∧' : '∨' }}
                                  </span>                                
                                </div>
                                <div class="accordion-item-body" role="region" [style.display]="ClassAccordionItem.expanded ? '' : 'none'">
                                  <div class="px-3 py-2 text-sm text-slate-500">
                                    <span class="text-sm font-medium text-slate-700 ">
                                      {{"Definition:"}}
                                    </span> 
                                    <br> 
                                    {{this.dependencyDetails()?.definition}}
                                  </div>
                                  <div class="px-3 py-2 text-sm text-slate-500">
                                    <span class="text-sm font-medium text-slate-700 ">
                                      {{"Axioms: "}} <br>
                                    </span> 
                                    @for (p of this.dependencyDetails()?.superclasses; track $index) {
                                      <span class="text-sm font-medium"> {{"SubclassOf "}} </span>
                                      <app-class-expression [expression]="p" [labelMap]="ontologyData().labels"/> <br>
                                    }
                                    @for (p of this.dependencyDetails()?.equivalentTo; track $index) {
                                      <span class="text-sm font-medium"> {{"EquivalentTo "}} </span>
                                      <app-class-expression [expression]="p" [labelMap]="ontologyData().labels"/> <br>
                                    }
                                  </div>
                                </div>
                              </div>
                            </cdk-accordion-item>
                          } 
                        } @else {
                          <span class="text-gray-500">
                            There are no added dependencies.
                          </span>
                        }
                      </cdk-accordion>
                    </div>
                  </div>
                </div>
                </cdk-accordion-item>  

                <cdk-accordion-item #accordionItem2="cdkAccordionItem">
                  <div class="mt-2 rounded-md border border-slate-200">
                    @let hasDeletedDeps = this.right_not_left().get({symbol_type: 'Class', value: iri});
                    <div class="px-3 py-2">
                      <b [class.text-slate-500]="!hasDeletedDeps">{{"Deleted Dependencies"}}</b>
                      <span [class.text-slate-500]="!hasDeletedDeps" class="toggle" style="float: right" (click)="accordionItem2.toggle()">
                        Click to {{ accordionItem2.expanded ? 'close' : 'open' }}
                      </span>
                    </div>
                    <div class="accordion-item-body" role="region" [style.display]="accordionItem2.expanded ? '' : 'none'">
                      <div class="px-3 py-2">
                        <cdk-accordion class="class-accordion">
                          @if(hasDeletedDeps){
                            @for (dep of hasDeletedDeps; track $index) {
                              <cdk-accordion-item #ClassAccordionItem ="cdkAccordionItem" (opened)="depDetails(dep.value)">
                                <div class="mt-2 rounded-md border border-slate-200">
                                  <div class="px-3 py-2"  style="font-weight: 600">
                                    <app-class-expression [expression]="{'type':'Class','iri':dep.value}" [labelMap]="ontologyData().labels"
                                                          (onClassClick)="onClassClick.emit($event)"
                                                          (onObjectPropertyClick)="onObjectPropertyClick.emit($event)"/>    
                                    <span class="toggle" (click)="ClassAccordionItem.toggle()" style="float: right">
                                      {{ ClassAccordionItem.expanded ? '∧' : '∨' }}
                                    </span>                                
                                  </div>
                                  <div class="accordion-item-body" role="region" [style.display]="ClassAccordionItem.expanded ? '' : 'none'">
                                    <div class="px-3 py-2 text-sm text-slate-500">
                                      <span class="text-sm font-medium text-slate-700 ">
                                        {{"Definition:"}}
                                      </span> 
                                      <br> 
                                      {{this.dependencyDetails()?.definition}}
                                    </div>
                                    <div class="px-3 py-2 text-sm text-slate-500">
                                      <span class="text-sm font-medium text-slate-700 ">
                                        {{"Axioms: "}} <br>
                                      </span> 
                                      @for (p of this.dependencyDetails()?.superclasses; track $index) {
                                        <span class="text-sm font-medium"> {{"SubclassOf "}} </span>
                                        <app-class-expression [expression]="p" [labelMap]="ontologyData().labels"/> <br>
                                      }
                                      @for (p of this.dependencyDetails()?.equivalentTo; track $index) {
                                        <span class="text-sm font-medium"> {{"EquivalentTo "}} </span>
                                        <app-class-expression [expression]="p" [labelMap]="ontologyData().labels"/> <br>
                                      }
                                    </div>
                                  </div>
                                </div>
                              </cdk-accordion-item>
                            } 
                          } @else {
                            <span class="text-gray-500">
                              There are no deleted dependencies.
                            </span>
                          }
                        </cdk-accordion>
                      </div>
                    </div>
                  </div>
                </cdk-accordion-item>

              </cdk-accordion>

            }

          </div>         
        </div> 
      </div>
      }
      <ng-template #empty>
        <div class="h-full flex items-center justify-center text-slate-500">
          <div>
            <h3 class="text-lg font-medium">No class selected</h3>
            <p class="text-sm">Select a class from the hierarchy to view details.</p>
          </div>
        </div>
      </ng-template>
    </div>
  `,
})


export class ClassDetailComponent implements OnChanges{
  @Input() iri!: string;
  @Input() ontologyData!: Signal<OntologyData>;
  @Input() left_not_right!: Signal<CompoundMap<any, any>>
  @Input() right_not_left!: Signal<CompoundMap<any, any>>
  @Output() onClassClick: EventEmitter<string> = new EventEmitter();
  @Output() onObjectPropertyClick: EventEmitter<string> = new EventEmitter();

  protected readonly classDetails = signal<ClassDetailsService | null>(null);
  protected readonly loading = signal(false);
  private currentIri = signal<string | null>(null);


  constructor() {}

  private effectRef = effect(() => {
    const iri = this.currentIri();
    if (iri !== null) {
      this.loading.set(true);
      invoke('get_class_details', {s:iri})
          .then(result => {
            let cd = new ClassDetailsService(result);
            cd.dependsOn = this.ontologyData().dependencies.get({'symbol_type':"Class", 'value':iri!}) || []
            this.classDetails.set(cd);
            this.loading.set(false);
          });
    }
  });

  protected readonly dependencyDetails = signal<ClassDetailsService | null>(null);
  depDetails(dep: string){
      invoke('get_class_details', { s: dep })
        .then(result => {
          const depDet = new ClassDetailsService(result);
              this.dependencyDetails.set(depDet);
        });
  }

  protected displayAllDeps = false;
  onDepChange() { 
    this.displayAllDeps = !this.displayAllDeps;
  }

  

  ngOnDestroy() {
    this.effectRef.destroy();
  }



  ngOnChanges(changes: SimpleChanges) {
    let iri = changes["iri"]?.currentValue;
    if (iri) {
      this.currentIri.set(iri || null);
    }
  }

  protected readonly console = console;

}


