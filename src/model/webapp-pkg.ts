export type WebappPkg = {
    package: {
        name: string,
        appname: string,
        version: string,
        authors: string[],
        entry: string,
        contact?: string[],
        brief?: string,
        repo?: string
    },
    args: {
        number?: WebappPkgArg<number>[]
        string?: WebappPkgArg<string>[]
        boolean?: WebappPkgArg<boolean>[]
    }

}

export type WebappPkgArg<T extends string|number|boolean> = {
    key:string,
    default: T,
    description: string
}


