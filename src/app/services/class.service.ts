import {signal} from "@angular/core";
import {invoke} from "@tauri-apps/api/core";

export interface ClassDetails {
    iri: string | null;
    subclassOf: string[];
    superclassOf: [],
    primary_label: string | null;
    annotations: Map<string, string[]>;
}

export class ClassService {
    readonly data = signal<ClassDetails>({
        iri: null,
        subclassOf: [],
        superclassOf: [],
        primary_label: null,
        annotations: new Map(),
    });

    constructor(cls: string | null) {
        invoke('load_class_details', {iri: cls}).then(
            (result) => this.data.set({
                iri: cls,
                subclassOf: [],
                superclassOf: [],
                primary_label: null,
                annotations: new Map()
            })
        )
    }
}