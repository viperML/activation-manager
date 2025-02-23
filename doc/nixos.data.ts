import child_process from "node:child_process";
import util from "node:util";
import fs from "node:fs/promises";
import { defineLoader } from 'vitepress'

export interface OptionEntry {
  declarations: string[],
  description: string,
  loc: string[],
  readOnly: boolean,
  type: string,
}

type Data = OptionEntry[]

declare const data: Data
export { data }

export default defineLoader({
  async load(): Promise<Data> {
    const exec = util.promisify(child_process.exec);
    const { stdout } = await exec("nix build -f ../test/main.nix config.build.optionsDoc.optionsJSON --no-link --print-out-paths")
    const file = `${stdout.trim()}/share/doc/nixos/options.json`;
    console.log(file);

    const data = await fs.readFile(file, { encoding: 'utf8' })

    const obj = JSON.parse(data);

    return Object.values(obj)
  }
})
