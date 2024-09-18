export const page = (html: { toString(): string }) => {
    return new Response(
        html.toString(),
        {
            status: 200,
            headers: { "Content-Type": "text/html" },
        },
    );
};
