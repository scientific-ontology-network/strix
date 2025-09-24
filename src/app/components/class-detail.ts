import {Component, Input, signal} from '@angular/core';
import { CommonModule } from '@angular/common';
import {invoke} from "@tauri-apps/api/core";
import {OntologyData} from "../services/ontology.service";

@Component({
  selector: 'app-class-detail',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="p-6">
      <ng-container *ngIf="cls; else empty">
        <div class="flex items-start justify-between gap-4">
          <div>
            <h2 class="text-2xl font-semibold text-slate-900">{{ this.labels[0] }}</h2>
            <p class="text-sm text-slate-500 mt-1" *ngIf="cls">{{ cls }}</p>
          </div>
        </div>

        <p class="mt-4 text-slate-700" *ngIf="this.definition">{{ this.definition }}</p>

        <div class="mt-6 grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <h3 class="text-sm font-medium text-slate-600 uppercase tracking-wider">Properties</h3>
            <div class="mt-2 rounded-md border border-slate-200 divide-y">
              <div *ngIf="this.annotations" class="px-3 py-2 text-sm text-slate-500">No properties</div>
              <div *ngFor="let anno of this.annotations" class="px-3 py-2">
                <div class="text-sm font-medium text-slate-800">{{ anno[0] }}</div>
                <div *ngFor="let v of anno[1]" class="px-3 py-2">
                  <div class="text-xs text-slate-500" *ngIf="v">{{v}}</div>
                </div>
              </div>
            </div>
          </div>
          <div>
            <h3 class="text-sm font-medium text-slate-600 uppercase tracking-wider">Metadata</h3>
            <div class="mt-2 rounded-md border border-slate-200">
              <div class="px-3 py-2 text-sm flex justify-between"><span class="text-slate-500">ID</span><span class="font-mono text-slate-800">{{ cls }}</span></div>
              <div class="border-t px-3 py-2 text-sm" *ngIf="this.parents?.length"><span class="text-slate-500">Parents:</span>
                <span class="ml-2 inline-flex gap-1 flex-wrap">
                  <span *ngFor="let p of this.parents" class="px-2 py-0.5 rounded bg-slate-100 text-slate-700 text-xs">{{ p }}</span>
                </span>
              </div>
            </div>
          </div>
        </div>
      </ng-container>
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


export class ClassDetailComponent {
  @Input() cls: string | null = null;
  @Input() ontologyData: OntologyData | null = null;

  protected classDependencies: string[] = [];
  protected annotations: Map<string, string[]> | null = null;
  protected definition: string | null = null;
  protected parents: string[] = [];
  protected labels: string[] = [];

  constructor() {
    if(this.cls)
    {
      this.classDependencies = this.ontologyData?.classDependencies.get(this.cls) ?? [];
      this.parents = this.ontologyData?.directSubclasses.get(this.cls) ?? [];
      this.annotations = this.ontologyData?.annotations.get(this.cls) ?? new Map();
      this.labels = this.annotations?.get("http://www.w3.org/2000/01/rdf-schema#label") ?? [this.cls];
    }

  }

}
