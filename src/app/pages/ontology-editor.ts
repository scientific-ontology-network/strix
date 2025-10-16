import {Component, computed, inject} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormsModule} from '@angular/forms';
import {ClassHierarchyComponent} from '../components/class-hierarchy';
import {ClassDetailComponent} from '../components/class/class-detail';
import {OntologyService} from '../services/ontology.service';
import {StorageService} from '../services/storage.service';
import {invoke} from "@tauri-apps/api/core";

// @ts-ignore
@Component({
  selector: 'app-ontology-editor',
  standalone: true,
  imports: [CommonModule, FormsModule, ClassHierarchyComponent, ClassDetailComponent],
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
              <button class="btn" (click)="open()">Open…</button>
            </nav>
          </div>
        </div>
      </header>

      <!-- Content -->
      <main class="container mx-auto px-4 py-6">
        <div class="grid grid-cols-1 lg:grid-cols-[320px_minmax(0,1fr)] gap-6">
          <aside class="rounded-xl border border-slate-200 bg-white/80 backdrop-blur">
            <div class="p-3 border-b border-slate-200">
              <input type="text" [ngModel]="svc.searchQuery()" (ngModelChange)="svc.searchQuery.set($event)" placeholder="Search classes…"
                     class="w-full rounded-md border-slate-300 focus:border-primary-500 focus:ring-primary-500 text-sm" />
            </div>
            <app-class-hierarchy
              [ontologyData]="svc.ontologyData()"
              [selectedId]="svc.selectedClassId()"
              [searchQuery]="svc.searchQuery()"
              (selected)="svc.select($event)"
            />
          </aside>
          <section class="rounded-xl border border-slate-200 bg-white/80 backdrop-blur min-h-[420px]">
            @if (selected()) {
              <app-class-detail 
                  [iri]="selected()!"
                  [ontologyData]="svc.ontologyData"
                  (onIriClick)="svc.select($event)"
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
  styles: [
    `.btn{ @apply px-3 py-1.5 rounded-md border border-slate-300 bg-white hover:bg-slate-50 text-sm shadow-sm transition; }
     .btn-primary{ @apply bg-primary-600 text-white border-primary-600 hover:bg-primary-700; }
     :host{ display:block }
    `,
  ],
})
export class OntologyEditorComponent {
  readonly svc = inject(OntologyService);
  readonly storage = inject(StorageService);

  readonly selected = computed(() => {
    return this.svc.selectedClassId();
  });

  async open() {
    const path = await this.storage.openTextFile();
    if (!path) return;
    try {
      this.svc.load(path);
    } catch (e: any) {
      alert('Failed to load ontology: ' + (e?.message ?? e));
    }
  }

}
