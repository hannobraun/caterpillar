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
