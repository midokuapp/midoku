import { Link } from "react-router-dom";

import { useExtensions } from "../../context/extensions.ts";

export default function Browse() {
  const { extensions } = useExtensions();

  return (
    <div>
      <div>
        <h1>
          Explore
        </h1>
      </div>

      <ul className="flex flex-col gap-2 px-8">
        {extensions.map((extension) => (
          <li
            key={extension.id}
            className="flex items-center gap-3"
          >
            <img
              src={extension.iconUrl}
              className="w-12 h-12"
              alt={`${extension.source.name} icon`}
            />
            <div>
              <span className="block text-lg">
                {extension.source.name}
              </span>
              <div className="text-sm opacity-70">
                {extension.source.language}
              </div>
            </div>
            <Link
              to={`/browse/${extension.id}`}
              className="ml-auto text-blue-600 hover:underline"
            >
              Browse
            </Link>
          </li>
        ))}
      </ul>
    </div>
  );
}
