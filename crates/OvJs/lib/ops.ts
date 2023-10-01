

globalThis.print = (value: any) => {
    if (value.constructor == String) {
        Deno.core.print(value + "\n");
    }
    else {
        Deno.core.print(value.toString() + "\n");
    }
}