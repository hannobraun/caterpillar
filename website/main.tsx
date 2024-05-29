import { dailyThoughtsPage } from "./code/templates.tsx";

Deno.serve(async (request) => {
    const url = new URL(request.url);

    if (url.hostname == "caterpillar.deno.dev") {
        return Response.redirect(
            "https://capi.hannobraun.com/",
            308,
        );
    }

    if (url.pathname == "/") {
        return Response.redirect(
            `${url.origin}/daily`,
            307,
        );
    }
    if (url.pathname == "/daily/") {
        return Response.redirect(
            `${url.origin}/daily`,
            307,
        );
    }

    if (url.pathname == "/daily") {
        const dates = [];
        for await (const dirEntry of Deno.readDir("daily")) {
            const date = dirEntry.name.match(
                /^(\d{4}-\d{2}-\d{2}).md$/,
            );

            if (date) {
                dates.push(date[1]);
            }
        }

        const page = dailyThoughtsPage(dates);

        return new Response(
            page.toString(),
            {
                status: 200,
                headers: new Headers([["Content-Type", "text/html"]]),
            },
        );
    }

    const dailyDateWithSlash = url.pathname.match(
        /^\/daily\/(\d{4}-\d{2}-\d{2})\/$/,
    );
    if (dailyDateWithSlash && dailyDateWithSlash[1]) {
        return Response.redirect(
            `${url.origin}/daily/${dailyDateWithSlash[1]}`,
            307,
        );
    }

    const dailyDateWithNoSlash = url.pathname.match(
        /^\/daily\/(\d{4}-\d{2}-\d{2})$/,
    );
    if (dailyDateWithNoSlash && dailyDateWithNoSlash[1]) {
        const path = `daily/${dailyDateWithNoSlash[1]}.md`;
        const file = await Deno.readTextFile(path);
        return new Response(file, { status: 200 });
    }

    return new Response("not found", { status: 404 });
});
