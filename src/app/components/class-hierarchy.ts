import { Component, EventEmitter, Input, Output, Signal, computed, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import {OntologyData} from '../services/ontology.service';

interface NodeVM { id: string; label: string; }

@Component({
  selector: 'app-class-hierarchy',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="p-3 flex flex-col h-full">
      <div class="text-xs text-slate-500 mb-2" *ngIf="!this.ontologyData.roots?.length">No classes loaded.</div>
      <ul class="space-y-1 flex-1 overflow-y-auto" *ngIf="this.ontologyData.roots?.length">
        <ng-container *ngFor="let id of this.ontologyData.roots; trackBy: track">
          <ng-template [ngTemplateOutlet]="nodeTpl" [ngTemplateOutletContext]="{ id: id, depth: 0 }" />
        </ng-container>
      </ul>
    </div>

    <ng-template #nodeTpl let-id="id" let-depth="depth">
      <li class="group relative">
        <div class="flex items-center gap-1 rounded px-1 py-0.5 cursor-pointer select-none relative
             before:content-[''] before:absolute before:top-1/2 before:-left-3 before:w-3 before:border-t before:border-slate-300"
             [class.bg-primary-50]="selectedId===id"
             [style.paddingLeft.px]="depth"
             (click)="select(id)">
          <button class="size-4 flex items-center justify-center text-slate-500 hover:text-slate-700"
                  (click)="toggle(id); $event.stopPropagation()"
                  *ngIf="children(id).length>0">
            <svg class="size-3 transition-transform" [class.rotate-90]="expanded.has(id)" viewBox="0 0 20 20" fill="currentColor"><path d="M7 5l6 5-6 5V5z"/></svg>
          </button>
          <span class="text-sm text-slate-800 truncate" [class.font-semibold]="selectedId===id">{{ label(id) }}</span>
        </div>
        <ul *ngIf="expanded.has(id)" class="ml-2 pl-3 border-l border-slate-300 space-y-0.5">
          <ng-container *ngFor="let cid of children(id); trackBy: track">
            <ng-template [ngTemplateOutlet]="nodeTpl" [ngTemplateOutletContext]="{ id: cid, depth: depth+1 }" />
          </ng-container>
        </ul>
      </li>
    </ng-template>
  `,
})
export class ClassHierarchyComponent {
  @Input() ontologyData!: OntologyData;
  @Input() selectedId: string | null = null;
  @Input() searchQuery = '';
  @Output() selected = new EventEmitter<string>();
  expanded = new Set<string>();

  private matches = computed(() => {
    const q = (this.searchQuery || '').trim().toLowerCase();
    if (!q) return new Set<string>();
    const set = new Set<string>();
    for (const c of this.ontologyData.roots) if (this.label(c).toLowerCase().includes(q) || (c ?? '').toLowerCase().includes(q)) set.add(c);
    return set;
  });

  ngOnChanges() {
    // Expand paths to matches when searching
    const matches = this.matches();
    if (matches.size > 0) {
      for (const id of matches) this.expandAncestors(id);
    }
  }

  track = (_: number, id: string) => id;
  protected label(id: string){
    let label = this.ontologyData.labels.get(id)?? id;
    //console.log(id, label);
    return label;
  }

  children(id: string): string[] {
    const out = this.ontologyData.directSubclasses.get(id) ?? [];
    const q = (this.searchQuery || '').trim().toLowerCase();
    if (!q) return out;

    // filter: include if child matches or has descendant match
    return out.filter(cid => this.isVisible(cid, q));
  }

  private isVisible(c: string, q: string): boolean {
    if (!c) return false;
    const here = this.label(c)?.toLowerCase().includes(q) || (c ?? '').toLowerCase().includes(q);
    if (here) return true;
    const kids = this.ontologyData.directSubclasses.get(c) ?? [];
    return kids.some(k => this.isVisible(k, q));
  }

  toggle(id: string) {
    if (this.expanded.has(id)) this.expanded.delete(id); else this.expanded.add(id);
  }

  select(id: string) {
    this.selected.emit(id);
  }

  private expandAncestors(id: string) {
    // naive ancestor expansion by reverse scanning parents from byId
    // Build reverse map on-the-fly
    const parentMap = new Map<string, string[]>();
    for (const c of this.ontologyData.roots) for (const p of this.ontologyData.directSubclasses.get(c) ?? []) {
      if (!parentMap.has(c)) parentMap.set(c, []);
      parentMap.get(c)!.push(p);
    }
    const visit = (n: string) => {
      const parents = parentMap.get(n) ?? [];
      for (const p of parents) {
        this.expanded.add(p);
        visit(p);
      }
    };
    visit(id);
  }
}
