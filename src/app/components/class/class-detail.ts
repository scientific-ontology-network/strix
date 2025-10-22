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
import {ClassExpressionComponent, ObjectPropertyExpressionComponent} from "./class-expression";
import {LiteralView} from "../../bindings/LiteralView";
import {v4 as uuidv4} from 'uuid';
import {OntologySymbolView} from "../../bindings/OntologySymbolView";



@Component({
  selector: 'app-class-detail',
  standalone: true,
  imports: [CommonModule, ClassExpressionComponent, ObjectPropertyExpressionComponent],
  template: `
    <div class="p-6">
      @if (loading()) {
        <p>Loading...</p>
      } @else {
        <div class="flex items-start justify-between gap-4">
          <div>
            <h2 class="text-2xl font-semibold text-slate-900">{{ ontologyData().labels.get(this.iri) ?? this.iri }}</h2>
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
                  <div class="text-sm font-medium text-slate-800">{{ anno[0] }}</div>
                  @for(v of anno[1]; track $index) {
                    <div class="px-3 py-2">
                      <div class="text-xs text-slate-500">{{ renderLiteral(v, ontologyData().labels) }}</div>
                    </div>
                  }
                </div>
                }
              }
            </div>
          </div>
          <div>
            <h3 class="text-sm font-medium text-slate-600 uppercase tracking-wider">Metadata</h3>
            <div class="mt-2 rounded-md border border-slate-200">
              @if(this.classDetails()?.subclasses?.length){
                <div class="border-t px-3 py-2 text-sm"><span
                    class="text-slate-500">Subclasses:</span>
                  <span class="ml-2 inline-flex gap-1 flex-wrap">
                    <ul>
                    @for (p of this.classDetails()?.subclasses; track $index) {
                      <li><app-class-expression [expression]="p" [labelMap]="ontologyData().labels"
                                                (onIriClick)="onIriClick.emit($event)"/></li>
                    }
                    </ul>
                  </span>
                </div>
              }
              @if(this.classDetails()?.superclasses?.length){
                <div class="border-t px-3 py-2 text-sm"><span
                    class="text-slate-500">Superclasses:</span>
                  <span class="ml-2 inline-flex gap-1 flex-wrap">
                    <ul>
                    @for (p of this.classDetails()?.superclasses; track $index) {
                      <li><app-class-expression [expression]="p" [labelMap]="ontologyData().labels"
                                                (onIriClick)="onIriClick.emit($event)"/></li>
                    }
                    </ul>
                  </span>
                </div>
              }
              <div class="border-t px-3 py-2 text-sm" *ngIf="this.classDetails()?.dependsOn?.length"><span
                  class="text-slate-500">Depends On:</span>
                <span class="ml-2 inline-flex gap-1 flex-wrap">
                  <ul class="flex flex-wrap gap-2"
                  >
                  @for (dependency of this.classDetails()?.dependsOn; track $index){
                    <li>
                      @switch (dependency.symbol_type) {
                        @case('CE') {
                          @if (dependency.type == "Class") {
                            <app-class-expression class="inline-flex items-center rounded-full bg-slate-100 px-3 py-1 text-sm"
                                                   [expression]="dependency" [labelMap]="ontologyData().labels"
                                                      (onIriClick)="onIriClick.emit($event)"/>
                          }
                        } @case('Role') {
                          @if (dependency.type == "ObjectProperty") {
                            <app-object-property-expression class="inline-flex items-center rounded-full bg-slate-100 px-3 py-1 text-sm"
                                                            [expression]="dependency" [labelMap]="ontologyData().labels"
                                                                (onIriClick)="onIriClick.emit($event)"/>
                          }
                        }
                      }
                    </li>
                  }</ul>
                </span>
              </div>
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
  @Output() onIriClick: EventEmitter<string> = new EventEmitter();

  protected readonly classDetails = signal<ClassDetailsService | null>(null);
  protected readonly loading = signal(false);
  private currentIri = signal<string | null>(null);
  private label = (iri: string) => this.ontologyData().labels.get(iri) ?? iri;

  constructor() {}

  private effectRef = effect(() => {
    const iri = this.currentIri();
    if (iri !== null) {
      this.loading.set(true);
      invoke('get_class_details', {iri})
          .then(result => {
            this.classDetails.set(new ClassDetailsService(result));
            this.loading.set(false);
          });
    }
  });

  ngOnDestroy() {
    this.effectRef.destroy();
  }



  ngOnChanges(changes: SimpleChanges) {
    let iri = changes["iri"]?.currentValue;
    if (iri) {
      this.currentIri.set(iri || null);
    }
  }

  protected readonly renderLiteral = renderLiteral;

  protected readonly console = console;
}

function renderLiteral(literal: LiteralView, labelMap: Map<string, string>): string {
  switch (literal.type) {
    case 'Simple':
      return `"${literal.value}"`;
    case 'Language':
      return `"${literal.value}"@${literal.language}`;
    case 'Datatype':
      return `"${literal.value}"^^${literal.datatype}`;
    default:
      return '<unknown literal>';
  }
}