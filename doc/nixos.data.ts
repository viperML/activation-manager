import child_process from "node:child_process";
import util from "node:util";
import fs from "node:fs/promises";
import { createMarkdownRenderer, defineLoader, type SiteConfig } from 'vitepress'

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
const config = globalThis.VITEPRESS_CONFIG as SiteConfig
const md = await createMarkdownRenderer(config.srcDir, config.markdown, config.site.base, config.logger)

    const exec = util.promisify(child_process.exec);
    const { stdout } = await exec("nix build -f ../test/main.nix config.build.optionsDoc.optionsJSON --no-link --print-out-paths")
    const file = `${stdout.trim()}/share/doc/nixos/options.json`;
    console.log(file);

    const data = await fs.readFile(file, { encoding: 'utf8' })

    const obj: Record<string, OptionEntry> = JSON.parse(data);

    const elems = Object.values(obj).map(elem => {
      elem.description = md.render(elem.description);

      return elem;
    });

    return Object.values(obj)
  }
})
