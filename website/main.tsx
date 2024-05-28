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

        dates.sort();
        dates.reverse();

        const entries = [];
        for (const date of dates) {
            const link = `/daily/${date}`;
            entries.push(
                <li class="my-4 font-bold text-lg">
                    <a href={link}>
                        {date}
                    </a>
                </li>,
            );
        }

        const css = `
            html {
                font-family: sans-serif;
            }
            a {
                color: #0000ff;
            }
            ol {
                list-style-type: none;

                margin: 0;
                padding: 0;
            }

            .font-bold {
                font-weight: 700;
            }
            .mx-auto {
                margin-left: auto;
                margin-right: auto;
            }
            .my-4 {
                margin-top: 1rem;
                margin-bottom: 1rem;
            }
            .text-lg {
                font-size: 1.125rem;
            }
            .w-fit {
                width: fit-content;
            }
        `;

        const page = (
            <>
                {"<!doctype html>"}
                <html>
                    <head>
                        <title>Daily Thoughts - Caterpillar</title>
                        <style>{css}</style>
                    </head>
                    <body class="w-fit mx-auto">
                        <ol>{entries}</ol>
                    </body>
                </html>
            </>
        );

        return new Response(
            page,
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
