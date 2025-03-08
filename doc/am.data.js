import { loadOptions, stripNixStore } from "easy-nix-documentation/loader"
export default {
    async load() {
        const optionsJSON = process.env.OPTIONS_JSON
        if (optionsJSON === undefined) {
            console.log("OPTIONS_JSON is undefined");
            exit(1)
        }
        return await loadOptions(optionsJSON, {
            mapDeclarations: declaration => {
                const relDecl = stripNixStore(declaration);
                return `<a href="https://github.com/viperML/activation-manager/tree/master/${relDecl}">&lt;activation-manager/${relDecl}&gt;</a>`
            },
        })
    }
}
