import { Injectable, signal, computed } from '@angular/core';
import {invoke} from "@tauri-apps/api/core";
import {toArray} from "rxjs";
import {AnnotationValueView} from "../bindings/AnnotationValueView";

export interface OntologyData {
  labels: Map<string, AnnotationValueView>;
  roots: string[];
  isAssertedSuperclassOf: Map<string, string[]>;
}

@Injectable({ providedIn: 'root' })
export class OntologyService {
  readonly ontologyData = signal<OntologyData>({
    labels: new Map(),
    roots: [],
    isAssertedSuperclassOf: new Map(),
  });
  readonly selectedClassId = signal<string | null>(null);
  readonly searchQuery = signal('');

  readonly roots = computed(() => {
    const d = this.ontologyData();
    if (!d) return [] as string[];
    return d.roots;
  });

  load(path: string) {
      invoke('load_ontology', {path: path}).then(
        // @ts-ignore
        ((classHierarchy) => {
          // @ts-ignore
          let labelMap = new Map(Object.entries(classHierarchy.labels));
          // @ts-ignore
          let directSubclasses =  new Map(Object.entries(classHierarchy.is_asserted_superclass_of));

          // @ts-ignore
          this.ontologyData.set({
            // @ts-ignore
            labels: labelMap,
            // @ts-ignore
            roots: classHierarchy.roots,
            // @ts-ignore
            isAssertedSuperclassOf: directSubclasses,
          });
          // reset selection
          // @ts-ignore
          const first = classHierarchy.roots[0] ?? null;
          this.selectedClassId.set(first);

        })
    )
  }


  clear() {
    this.ontologyData.set({
      isAssertedSuperclassOf: new Map(),
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
