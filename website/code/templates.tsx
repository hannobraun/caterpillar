import { JSX } from "@bossley9/sjsx/jsx-runtime";
import * as gfm from "@deno/gfm";

export const dailyThoughtsPage = (dates: string[]) => {
    const entries = [];
    for (const date of dates) {
        entries.push(dailyThoughtItem(date));
    }

    return page(
        "Daily Thoughts",
        <>
            <h2>Daily Thoughts</h2>
            <p class="prose">
                Hey, I'm Hanno! These are my daily thoughts on{" "}
                <a href="https://github.com/hannobraun/caterpillar">
                    Caterpillar
                </a>, the programming language I'm creating. If you have any
                questions, comments, or feedback, please{" "}
                <a href="mailto:hello@hannobraun.com">
                    get in touch
                </a>!
            </p>
            <ol class="m-8">{entries}</ol>
        </>,
    );
};

export const singleDailyThoughtPage = (
    date: string,
    md: string,
    dates: string[],
) => {
    const html = gfm.render(md, {
        allowedTags: ["source"],
        allowedAttributes: { "source": ["src"] },
    });

    const index = dates.findIndex((element) => element == date);

    const prev = dates[index + 1];
    const next = dates[index - 1];

    return page(
        `Daily Thought - ${date}`,
        <>
            <h2>Daily Thought - {date}</h2>
            {link("/daily", "< back to list")}
            <main class="prose">
                {html}
            </main>
            <div class="grid grid-cols-2">
                {prev && (
                    <span class="col-1 justify-self-start">
                        {dailyThoughtLink(prev, "<< previous thought")}
                    </span>
                )}
                {next && (
                    <span class="col-2 justify-self-end">
                        {dailyThoughtLink(next, "next thought >>")}
                    </span>
                )}
            </div>
        </>,
    );
};

const dailyThoughtItem = (date: string) => {
    const link = dailyThoughtLink(date, date);

    return (
        <li class="my-4 font-bold text-lg">
            {link}
        </li>
    );
};

const dailyThoughtLink = (date: string, label: string) => {
    const url = `/daily/${date}`;
    return link(url, label);
};

const link = (url: string, label: string) => {
    return (
        <a href={url} class="text-blue-700 underline">
            {label}
        </a>
    );
};

const page = (title: string, content: JSX.Element) => {
    return (
        <>
            {"<!doctype html>"}
            <html lang="en">
                <head>
                    <title>{title} - Caterpillar</title>

                    <meta charSet="UTF-8" />
                    <meta
                        name="viewport"
                        content="width=device-width, initial-scale=1"
                    />

                    <link href="/style.css" rel="stylesheet" />
                </head>
                <body class="max-w-xl mx-auto p-2">
                    <header>
                        <h1>Caterpillar</h1>
                    </header>
                    <main>
                        {content}
                    </main>

                    <hr class="w-1/2 mx-auto my-16" />

                    <footer class="max-w-fit mx-auto text-sm">
                        <p class="max-w-fit mx-auto italic">A website by</p>
                        <address>
                            <div>
                                Hanno Braun<br />
                                Untere Pfarrgasse 19<br />
                                64720 Michelstadt<br />
                                Germany<br />
                            </div>
                            <div class="my-4">
                                📧{" "}
                                <a href="mailto:hello@hannobraun.com">
                                    hello@hannobraun.com
                                </a>
                            </div>
                        </address>
                    </footer>
                </body>
            </html>
        </>
    );
};
