Deno.serve((request) => {
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
        return Response.redirect(
            "https://github.com/hannobraun/caterpillar/blob/main/daily.md",
            307,
        );
    }

    return new Response("not found", { status: 404 });
});
