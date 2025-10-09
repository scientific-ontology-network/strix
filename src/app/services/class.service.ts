import {ClassExpressionView} from "../bindings/ClassExpressionView";
import {OntologySymbolView} from "../bindings/OntologySymbolView";
import {LiteralView} from "../bindings/LiteralView";

export class ClassDetailsService {
    iri: string | null = null;
    annotations: Map<String, LiteralView[]> = new Map();
    subclasses: ClassExpressionView[] = [];
    superclasses: ClassExpressionView[] = [];
    equivalentTo: ClassExpressionView[] = [];
    disjointWith: ClassExpressionView[] = [];
    disjointUnionOf: ClassExpressionView[][] = [];
    individuals: String[] = [];
    definition: String | null = null;
    label: String | null = null;
    dependsOn: OntologySymbolView[] = [];

    public constructor(result:any) {
        if (result === null) return;
        // @ts-ignore
        this.annotations = new Map(Object.entries(result.annotations))
        // @ts-ignore
        this.subclasses = result.superclass_of
        // @ts-ignore
        this.superclasses = result.subclass_of
        // @ts-ignore
        this.equivalentTo = result.equivalent_to
        // @ts-ignore
        this.disjointWith = result.disjoint_with
        // @ts-ignore
        this.disjointUnionOf = result.disjoint_union_of
        // @ts-ignore
        this.individuals = result.individuals
        // @ts-ignore
        this.definition = result.definition
        // @ts-ignore
        this.label = result.label
        // @ts-ignore
        this.dependsOn = result.depends_on
    }

}
