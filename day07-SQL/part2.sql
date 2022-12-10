CREATE OR REPLACE FUNCTION array_reverse(anyarray) RETURNS anyarray AS $$
SELECT ARRAY(
    SELECT $1[i]
    FROM generate_subscripts($1,1) AS s(i)
    ORDER BY i DESC
);
$$ LANGUAGE 'sql' STRICT IMMUTABLE;



CREATE OR REPLACE FUNCTION array_slicer(anyarray int[], ind int)
	RETURNS int[]
	language plpgsql
	as
$$
declare
	arr1 int[] := anyarray;
	arrL int[];
	arrR int[];
	num int;
	catch int;
	finalL int;
	finalR int;
begin

	arrL := array_reverse(arr1[:ind-1]);
	arrR := arr1[ind+1:];
	catch := 0;
	finalL := 0;
	finalR := 0;
	
	<<"FOREACH1 eaxmple">>
	foreach num in array arrL loop
		IF catch = 0
			THEN finalL := finalL + 1;
		END IF;
		
		IF num >= arr1[ind] AND catch = 0
			THEN catch := 1;
		END IF;
	end loop "FOREACH1 eaxmple";
	
	catch := 0;
	
	<<"FOREACH2 eaxmple">>
	foreach num in array arrR loop		
		IF catch = 0
			THEN finalR := finalR + 1;
		END IF;
		
		IF num >= arr1[ind] AND catch = 0
			THEN catch := 1;
		END IF;
	end loop "FOREACH2 eaxmple";
		
	return array[finalL, finalR];
end;
$$;


SELECT array_slicer('{3,4,5,2,3,4,5,6,7}',6); --for testing



WITH coretable AS (
	SELECT *
	, DENSE_RANK() over (PARTITION by y1 ORDER BY count) as x1
	FROM (
		SELECT 
		*
		, row_number() over() as count
		FROM (
			SELECT 
				*, 
				unnest(arr) as value
			FROM (
				SELECT
					regexp_split_to_array(level1,'') as arr,
					row_number() over () as y1
				FROM (
					SELECT 
						TRIM(
							regexp_replace(
								unnest(
									string_to_array( 
										'30373
										25512
										65332
										33549
										35390'
										,
										E'\n'
									)
								)
								, '\s+|\t+','','g' 
							)
						) as level1
				) as foo
			) as foo2
		) as foo3
	) as foo4
)
, fulltable as (
	SELECT 
	 value,
	 y1,
	 x1,
	 count,
	 arr as arrX,
     array_agg(value) over(partition BY x1 ORDER by y1 ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING)::int[1] as arrY
	 FROM coretable
), slicys as (
	SELECT 
		*,
		array_slicer(arrX::int[],x1::int) as agg1,
		array_slicer(arrY::int[],y1::int) as agg2
	FROM fulltable
)
, allviews as (
	SELECT 
 		value,
 		y1,
		x1,
		count,
		agg1[1]::int as XL,
		agg1[2]::int as XR,
		agg2[1]::int as YT,
		agg2[2]::int as YB,
		agg1[1]::int * agg1[2]::int * agg2[1]::int * agg2[2]::int as viewcount
	FROM slicys
)
SELECT max(viewcount) FROM allviews;