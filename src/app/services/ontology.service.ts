import { Injectable, signal, computed } from '@angular/core';
import {invoke} from "@tauri-apps/api/core";
import {toArray} from "rxjs";

export interface OntologyData {
  classes: string[];
  annotations: Map<string, Map<string, string>>;
  roots: string[];
  directSubclasses: Map<string, string[]>;
  classDependencies: Map<string, string[]>;
}

@Injectable({ providedIn: 'root' })
export class OntologyService {
  readonly ontologyData = signal<OntologyData>({
    classes: [],
    annotations: new Map(),
    roots: [],
    directSubclasses: new Map(),
    classDependencies: new Map(),
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
        ([annotations, classes, roots, directSubclasses, classDependencies]) => {
          // @ts-ignore
          let annotationsMap = new Map(Object.entries(annotations).map(([k,vs],_) => [k, new Map(Object.entries({...vs}))]));
          this.ontologyData.set({
            classes,
            // @ts-ignore
            annotations: annotationsMap,
            roots,
            directSubclasses: new Map(Object.entries(directSubclasses)),
            classDependencies: new Map(Object.entries(classDependencies))
          });
          // reset selection
          const first = roots[0] ?? null;
          this.selectedClassId.set(first);


        }
    )
  }


  clear() {
    this.ontologyData.set({
      classes: [],
      annotations: new Map(),
      roots: [],
      directSubclasses: new Map(),
      classDependencies: new Map(),
    });
    // Todo: Clear ontology in backend
    this.selectedClassId.set(null);
  }

  select(id: string) {
    this.selectedClassId.set(id);
  }



}
