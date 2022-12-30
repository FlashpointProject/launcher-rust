import { Button } from '@mui/material';
import { useReactTable, createColumnHelper, getCoreRowModel, flexRender, Row } from '@tanstack/react-table';
import React from 'react';
import { User, UserContext } from './app';
import { GameOfTheDay } from './pages/Suggestions';

export type GotdTableProps = {
  data: Array<GameOfTheDay>;
  deleteGotd: (id: string) => void;
}

export function GotdTable(props: GotdTableProps) {
  const user = React.useContext(UserContext);

  const columnHelper = createColumnHelper<GameOfTheDay>();

  const columns = [
    columnHelper.accessor('id', {
      header: 'Game ID',
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
    columnHelper.accessor('date', {
      header: 'Date Submitted',
      footer: info => info.column.id,
      cell: cellProps => <span>{formatDateYMD(cellProps.getValue())}</span>,
    })
  ];

  if (user) {
    columns.push(columnHelper.display({
      id: 'actions',
      header: 'Actions',
      cell: cellProps => <RowActions user={user} deleteFunc={props.deleteGotd} row={cellProps.row} />,
    }));
  }

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

type RowActionsProps = {
  user: User;
  row: Row<GameOfTheDay>;
  deleteFunc: (id: string) => void
};

function RowActions(props: RowActionsProps) {
  return (
    <div>
      {props.user.admin && (
        <>
          <Button variant='contained' color='error' onClick={() => {
            props.deleteFunc(props.row.original.date);
          }}>Delete</Button>
        </>
      )}
    </div>
  );
}