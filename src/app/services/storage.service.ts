import {Injectable} from '@angular/core';
import {open} from '@tauri-apps/plugin-dialog';

// Minimal file save/open utilities supporting: Web File System Access API and optional Tauri global
@Injectable({ providedIn: 'root' })
export class StorageService {

  private _owl_extensions = ['.owl', '.owx', '.omn']
  private isTauri(): boolean {
    return typeof (window as any).__TAURI__ !== 'undefined';
  }

  async openTextFile(): Promise<string | null> {
    return await open({
      multiple: false,
      directory: false,
    });
  }

  async saveTextFile(content: string, suggestedName = 'ontology.owl'): Promise<void> {
      const api = (window as any).__TAURI__;
      const { save } = api.dialog;
      const { writeTextFile } = api.fs;
      const path = await save({ defaultPath: suggestedName, filters: [{ name: 'Ontology', extensions: this._owl_extensions }] });
      if (!path) return;
      await writeTextFile(path, content);
      return;
  }
}
