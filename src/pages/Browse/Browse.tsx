import { Link } from "react-router-dom";

import { useStore } from "../../services/store.service.ts";

export default function Browse() {
  const extensions = useStore((state) => state.extensions);

  const Icon = ({ src, alt }: { src: string; alt: string }) => (
    <figure className="w-12 h-12">
      <img src={src} alt={alt} className="rounded-md" />
    </figure>
  );

  const Detail = ({ children }: { children: React.ReactNode }) => (
    <div className="flex flex-col">{children}</div>
  );

  const Title = ({ title }: { title: string }) => (
    <h2 className="text-lg font-semibold">{title}</h2>
  );

  const Description = ({ language }: { language: string }) => (
    <p className="text-sm opacity-70">
      {language}
    </p>
  );

  const Item = (
    { key, children }: {
      key: string;
      children: React.ReactNode;
    },
  ) => (
    <li key={key} className="flex items-center gap-4 p-3 rounded-lg shadow-md">
      {children}
    </li>
  );

  const Group = ({ children }: { children: React.ReactNode }) => (
    <div className="space-y-4 mb-8">{children}</div>
  );

  return (
    <div className="max-w-xl mx-auto p-3">
      <h1 className="text-2xl font-bold mb-4">Explore</h1>

      <Group>
        {extensions.map((extension) => (
          <Item key={extension.id}>
            <Icon
              src={extension.iconUrl}
              alt={extension.source.name}
            />
            <Detail>
              <Title title={extension.source.name} />
              <Description language={extension.source.language} />
            </Detail>
            <Link
              to={`/browse/${extension.id}`}
              className="ml-auto btn hover:btn-primary"
            >
              Browse
            </Link>
          </Item>
        ))}
      </Group>
    </div>
  );
}
