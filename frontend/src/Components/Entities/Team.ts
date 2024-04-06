export enum Level {
  None = 0,
  Junior = 1,
  Recreational = 2,
  Competitive = 3
}

export enum Type {
  None = 0,
  Masculine = 1,
  Feminine = 2,
  Mixed = 3
}

export enum Sport {
  None = 0,
  Soccer = 1,
  Basketball = 2,
  Volleyball = 3,
  Baseball = 4,
  Football = 5,
}

type Team = {
  id: number;
  name: string;
  level: Level;
  type: Type;
  sport: Sport;
};


export default Team;


