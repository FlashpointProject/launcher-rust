import { Button } from '@mui/material';
import { useReactTable, createColumnHelper, getCoreRowModel, flexRender } from '@tanstack/react-table';

export type SuggestionsData = {
  id: string;
  game_id: string;
  title: string;
  description: string;
  author: string;
  date_submitted: string;
  assigned_dates: Array<string>;
}

export type SuggestionsTableProps = {
  data: Array<SuggestionsData>;
}

export function SuggestionsTable(props: SuggestionsTableProps) {
  const columnHelper = createColumnHelper<SuggestionsData>();

  const columns = [
    columnHelper.accessor('game_id', {
      header: 'Game ID',
      footer: info => info.column.id,
    }),
    columnHelper.accessor('title', {
      header: 'Title',
      footer: info => info.column.id,
    }),
    columnHelper.accessor('author', {
      header: 'Author',
      footer: info => info.column.id,
    }),
    columnHelper.accessor('description', {
      header: 'Description',
      footer: info => info.column.id,
    }),
    columnHelper.accessor('date_submitted', {
      header: 'Date Submitted',
      footer: info => info.column.id,
      cell: cellProps => <span>{formatDateYMDTime(cellProps.getValue())}</span>,
    }),
    columnHelper.accessor('assigned_dates', {
      header: 'Assigned Dates',
      footer: info => info.column.id,
      cell: cellProps => {
        const val = cellProps.getValue();
        if (val) {
          return <span>{val.length} {val.length === 1 ? 'Time' : 'Times'}</span>;
        } else {
          return <Button>DOG</Button>;
        }
      }
    })
  ];

  const table = useReactTable({ columns, data: props.data, getCoreRowModel: getCoreRowModel() });

  return (
    <table>
      <thead>
        {table.getHeaderGroups().map(headerGroup => (
          <tr key={headerGroup.id}>
            {headerGroup.headers.map(header => (
              <th key={header.id}>
                {header.isPlaceholder
                  ? null
                  : flexRender(
                    header.column.columnDef.header,
                    header.getContext()
                  )}
              </th>
            ))}
          </tr>
        ))}
      </thead>
      <tbody>
        {table.getRowModel().rows.map(row => (
          <tr key={row.id}>
            {row.getVisibleCells().map(cell => (
              <td key={cell.id}>
                {flexRender(cell.column.columnDef.cell, cell.getContext())}
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
}

function formatDateYMD(date: string) {
  const d = new Date(date);
  return `${d.getFullYear()}-${d.getMonth() + 1}-${d.getDate()}`;
}

function formatDateYMDTime(date: string) {
  const d = new Date(date);
  return `${d.getFullYear()}-${d.getMonth() + 1}-${d.getDate()} ${d.getHours()}:${d.getMinutes()}:${d.getSeconds()}`;
}