import { Injectable, signal, computed } from '@angular/core';
import {invoke} from "@tauri-apps/api/core";
import {toArray} from "rxjs";
import {AnnotationValueView} from "../bindings/AnnotationValueView";
import {OntologySymbolView} from "../bindings/OntologySymbolView";
import CompoundMap from "../util/compound-map"
export interface OntologyData {
  labels: Map<string, AnnotationValueView>;
  classRoots: string[];
  isAssertedSuperclassOf: Map<string, string[]>;

  objectPropertyRoots: string[];
  isAssertedSuperObjectPropertyOf: Map<string, string[]>;

  dependencies: Map<OntologySymbolView, OntologySymbolView[]>;
  dependencyRoots: OntologySymbolView[];
}

@Injectable({ providedIn: 'root' })
export class OntologyService {
  readonly ontologyData = signal<OntologyData>({
    labels: new Map(),
    classRoots: [],
    isAssertedSuperclassOf: new Map(),
    objectPropertyRoots: [],
    isAssertedSuperObjectPropertyOf: new Map(),
    dependencies: new Map(),
      dependencyRoots: [],
  });
  readonly selectedClassId = signal<string | null>(null);
  readonly selectedObjectPropertyId = signal<string | null>(null);
  readonly searchQuery = signal('');

  load(path: string) {
      invoke('load_ontology', {'path': path}).then(
        // @ts-ignore
        (([classHierarchy, dependencyMap, dependencyRoots]) => {

          // @ts-ignore
          let labelMap = new Map(Object.entries(classHierarchy.labels));
          // @ts-ignore
          let directSubclasses =  new Map(Object.entries(classHierarchy.is_asserted_superclass_of));
          // @ts-ignore
          let directSubproperties =  new Map(Object.entries(classHierarchy.is_asserted_super_object_property_of));
          // @ts-ignore
          let dependencies = new CompoundMap(dependencyMap);
          // @ts-ignore
          this.ontologyData.set({
            // @ts-ignore
            labels: labelMap,
            // @ts-ignore
            classRoots: classHierarchy.class_roots,
            // @ts-ignore
            isAssertedSuperclassOf: directSubclasses,
            // @ts-ignore
            objectPropertyRoots: classHierarchy.object_property_roots,
            // @ts-ignore
            isAssertedSuperObjectPropertyOf: directSubproperties,
              // @ts-ignore
              dependencies: dependencies,
              //@ts-ignore
              dependencyRoots: dependencyRoots
          });
          // @ts-ignore
          const first = classHierarchy.class_roots[0] ?? null;
          this.selectedClassId.set(first);

        })
    )
  }

  selectClass(id: string) {
    this.selectedClassId.set(id);
    this.selectedObjectPropertyId.set(null);
  }

  selectObjectProperty(id: string) {
    this.selectedClassId.set(null);
    this.selectedObjectPropertyId.set(id);
  }



}
