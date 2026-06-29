import {Component, computed, inject, signal, Signal} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormsModule} from '@angular/forms';
import {HierarchyTreeComponent} from '../components/hierarchy-tree';
import {ClassDetailComponent} from '../components/class/class-detail';
import {OntologyService} from '../services/ontology.service';
import {StorageService} from '../services/storage.service';
import {OntologySymbolView} from "../bindings/OntologySymbolView";
import {invoke} from "@tauri-apps/api/core";
import CompoundMap from "../util/compound-map";

// @ts-ignore
@Component({
  selector: 'app-ontology-editor',
  standalone: true,
  imports: [CommonModule, FormsModule, HierarchyTreeComponent, ClassDetailComponent],
  template: `
    <div class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 text-slate-900">
      <!-- Top Bar -->
      <header class="sticky top-0 z-20 backdrop-blur bg-white/70 border-b border-slate-200">
        <div class="container mx-auto px-4">
          <div class="flex items-center justify-between h-14">
            <div class="flex items-center gap-2">
              <div class="size-7 rounded bg-primary-600 flex items-center justify-center text-white font-bold">Ω</div>
              <div class="text-sm leading-tight">
                <div class="font-semibold tracking-tight">Strix</div>
                <div class="text-[10px] text-slate-500 -mt-0.5">Ontology Editor</div>
              </div>
            </div>
            <nav class="flex items-center gap-2">
              <button class="px-3 py-1.5 rounded-md border border-slate-300 bg-white hover:bg-slate-50 text-sm shadow-sm transition" (click)="open()">Open…</button>
            </nav>
            @if(svc.ontologyData().classRoots) {
              <nav class="flex items-center gap-2">
                <button class="px-3 py-1.5 rounded-md border border-slate-300 bg-white hover:bg-slate-50 text-sm shadow-sm transition" (click)="compare()">Compare</button>
              </nav>
            }
          </div>
        </div>
      </header>

      <!-- Content -->
      <main class="container mx-auto px-4 py-6">
        <div class="grid grid-cols-1 lg:grid-cols-[320px_minmax(0,1fr)] gap-6">
          <aside class="rounded-xl border border-slate-200 bg-white/80 backdrop-blur">
            <div class="p-3 border-b border-slate-200">
              <input type="text" [ngModel]="svc.searchQuery()" (ngModelChange)="svc.searchQuery.set($event)"
                     placeholder="Search classes…"
                     class="w-full rounded-md border-slate-300 focus:border-primary-500 focus:ring-primary-500 text-sm"/>
            </div>
            <div class="p-3 border-b border-slate-200">
              Classes
              <app-hierarchy-tree
                  [hierarchy]="svc.ontologyData().isAssertedSuperclassOf"
                  [roots]="svc.ontologyData().classRoots"
                  [labels]="svc.ontologyData().labels"
                  [selectedId]="svc.selectedClassId()"
                  [searchQuery]="svc.searchQuery()"
                  [stringify]="id"
                  (selected)="svc.selectClass($event)"
              />
            </div>
            <div class="p-3 border-b border-slate-200">
              Object Properties
              <app-hierarchy-tree
                  [hierarchy]="svc.ontologyData().isAssertedSuperObjectPropertyOf"
                  [roots]="svc.ontologyData().objectPropertyRoots"
                  [labels]="svc.ontologyData().labels"
                  [selectedId]="svc.selectedObjectPropertyId()"
                  [searchQuery]="svc.searchQuery()"
                  [stringify]="id"
                  (selected)="svc.selectObjectProperty($event)"
              />
            </div>
            <div class="p-3 border-b border-slate-200">
              Dependencies
              <app-hierarchy-tree
                  [hierarchy]="svc.ontologyData().dependencies"
                  [roots]="svc.ontologyData().dependencyRoots"
                  [labels]="svc.ontologyData().labels"
                  [selectedId]="svc.selectedClassId() || svc.selectedObjectPropertyId()"
                  [searchQuery]="svc.searchQuery()"
                  [stringify]="getSymbolString"
                  (selected)="selectDependencySymbol($event)"
              />
            </div>
          </aside>
          <section class="rounded-xl border border-slate-200 bg-white/80 backdrop-blur min-h-[420px]">
            @if (selected()) {
              <app-class-detail
                  [iri]="selected()!"
                  [ontologyData]="svc.ontologyData"
                  [left_not_right]="this.left_not_right"
                  [right_not_left]="this.right_not_left"
                  (onClassClick)="svc.selectClass($event)"
                  (onObjectPropertyClick)="svc.selectObjectProperty($event)"
              />
            }
          </section>
        </div>
        <div class="mt-4 text-xs text-slate-500">
          Tip: Works on web and Tauri. Use the menu above to load and save ontology JSON files.
        </div>
      </main>
    </div>
  `,
  styles: [`
     :host{ display:block }
    `,
  ],
})
export class OntologyEditorComponent {
  readonly svc = inject(OntologyService);
  readonly storage = inject(StorageService);

  left_not_right = signal<CompoundMap<unknown, unknown>>(new CompoundMap());
  right_not_left = signal<CompoundMap<unknown, unknown>>(new CompoundMap());

  readonly selected = computed(() => {
    return this.svc.selectedClassId();
  });

  selectDependencySymbol(s: OntologySymbolView){
    if (s.symbol_type === "Class") {
      this.svc.selectClass(s.value)
    } else if (s.symbol_type === "Role") {
      this.svc.selectObjectProperty(s.value)
    }
  }

  getSymbolString = (s: OntologySymbolView)=> s.value;

  id<S>(s:S) {
    return s
  }

  async open() {
    const path = await this.storage.openTextFile();
    if (!path) return;
    try {
      this.svc.load(path);
      this.svc.ontologyData().dependencies
    } catch (e: any) {
      alert('Failed to load ontology: ' + (e?.message ?? e));
    }
  }


  async compare() {
    const path = await this.storage.openTextFile();
    if (!path) return;
    try {
      invoke('dependency_diff', {'path': path}).then(
          // @ts-ignore
          (([raw_left_not_right, raw_right_not_left]) => {
            /*let left_not_right = new CompoundMap(raw_left_not_right);
            let right_not_left = new CompoundMap(raw_right_not_left);
            console.log(left_not_right, right_not_left)*/

            this.left_not_right.set(new CompoundMap(raw_left_not_right));
            this.right_not_left.set(new CompoundMap(raw_right_not_left));
            console.log("dep: " + new CompoundMap(raw_left_not_right), new CompoundMap(raw_right_not_left))
          })
      )
    } catch (e: any) {
      alert('Failed to load ontology: ' + (e?.message ?? e));
    }
  }

}
