import { Injectable, signal, computed } from '@angular/core';
import {invoke} from "@tauri-apps/api/core";
import {toArray} from "rxjs";

export interface OntologyData {
  labels: Map<string, string>;
  roots: string[];
  directSubclasses: Map<string, string[]>;
}

@Injectable({ providedIn: 'root' })
export class OntologyService {
  readonly ontologyData = signal<OntologyData>({
    labels: new Map(),
    roots: [],
    directSubclasses: new Map(),
  });
  readonly selectedClassId = signal<string | null>(null);
  readonly searchQuery = signal('');

  readonly roots = computed(() => {
    const d = this.ontologyData();
    if (!d) return [] as string[];
    return d.roots;
  });

  update() {

    invoke('get_ontology_structure').then(
        // @ts-ignore
        ([roots, directSubclasses,labels, ]) => {
          // @ts-ignore
          let labelMap = new Map(Object.entries(labels));
          this.ontologyData.set({
            // @ts-ignore
            labels: labelMap,
            roots: roots,
            directSubclasses: new Map(Object.entries(directSubclasses)),
          });
          // reset selection
          const first = roots[0] ?? null;
          this.selectedClassId.set(first);

        }
    )
  }


  clear() {
    this.ontologyData.set({
      directSubclasses: new Map(),
      labels: new Map(),
      roots: [],
    });
    // Todo: Clear ontology in backend
    this.selectedClassId.set(null);
  }

  select(id: string) {
    this.selectedClassId.set(id);
  }



}
