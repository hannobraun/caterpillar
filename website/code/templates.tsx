export const dailyThoughtItem = (date: string) => {
    const link = `/daily/${date}`;

    return (
        <li class="my-4 font-bold text-lg">
            <a href={link}>
                {date}
            </a>
        </li>
    );
};

export const css = `
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
