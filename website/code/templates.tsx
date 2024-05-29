import { JSX } from "@bossley9/sjsx/jsx-runtime";

export const dailyThoughtsPage = (dates: string[]) => {
    dates.sort();
    dates.reverse();

    const entries = [];
    for (const date of dates) {
        entries.push(dailyThoughtItem(date));
    }

    return page(
        "Daily Thoughts",
        <>
            <h2>Daily Thoughts</h2>
            <p>
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

const page = (title: string, content: JSX.Element) => {
    return (
        <>
            {"<!doctype html>"}
            <html>
                <head>
                    <title>{title} - Caterpillar</title>
                    <style>{css}</style>
                </head>
                <body class="max-w-xl mx-auto">
                    <h1>Caterpillar</h1>
                    {content}
                </body>
            </html>
        </>
    );
};

const dailyThoughtItem = (date: string) => {
    const link = `/daily/${date}`;

    return (
        <li class="my-4 font-bold text-lg">
            <a href={link}>
                {date}
            </a>
        </li>
    );
};

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
    .m-8 {
        margin: 2rem;
    }
    .max-w-xl {
        width: 36rem;
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
`;
