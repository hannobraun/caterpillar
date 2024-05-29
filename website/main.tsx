import * as content from "./code/content.ts";
import * as response from "./code/response.ts";
import { dailyThoughtsPage, singleDailyThoughtPage } from "./code/templates.tsx";

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
        const dates = await content.listDailyThoughts();
        const page = dailyThoughtsPage(dates);
        return response.page(page);
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
        const date = dailyDateWithNoSlash[1];
        const path = `daily/${date}.md`;
        const md = await Deno.readTextFile(path);
        const page = singleDailyThoughtPage(date, md);
        return response.page(page);
    }

    return new Response("not found", { status: 404 });
});
