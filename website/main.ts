Deno.serve((request) => {
    const url = new URL(request.url);

    if (url.pathname == "/daily") {
        return Response.redirect(
            "https://github.com/hannobraun/caterpillar/blob/main/daily.md",
            307,
        );
    }
    if (url.pathname == "/daily/") {
        return Response.redirect(
            `${url.origin}/daily`,
            307,
        );
    }

    return new Response("not found", { status: 404 });
});
