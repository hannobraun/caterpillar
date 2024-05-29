export const page = (html: { toString(): string }) => {
    return new Response(
        html.toString(),
        {
            status: 200,
            headers: new Headers([["Content-Type", "text/html"]]),
        },
    );
};
