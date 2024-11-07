export interface FilterTitle {
  query: string;
}

export interface FilterSort {
  optionIndex: number;
  optionReversed: boolean;
}

export type Filter = FilterTitle | FilterSort;
