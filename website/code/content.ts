export const listDailyThoughts = async () => {
    const dates = [];

    for await (const dirEntry of Deno.readDir("daily")) {
        const date = dirEntry.name.match(
            /^(\d{4}-\d{2}-\d{2}).md$/,
        );

        if (date) {
            dates.push(date[1]);
        }
    }

    return dates;
};
